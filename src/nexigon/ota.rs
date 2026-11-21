//! Firmware updates through the OTA update contract of Nexigon Hub.
//!
//! The device reads its desired state from the `dev.nexigon.ota.config`
//! property and reports its actual state in `dev.nexigon.ota.status`. The
//! property applies to defaults built into the firmware, such as
//! [`config::UPDATE_PATH`]. The
//! configuration names a version path `<repository>/<package>/<version>`. The
//! running firmware is the version of the package with the tag
//! [`crate::FIRMWARE_VERSION`]. If the path resolves to another version, the
//! device downloads its asset, `firmware.bin` by default, and the signature of
//! the asset, `firmware.bin.sig`, writes the asset to the DFU slot, and resets
//! into it. After the reset, the new firmware commits itself and reports the
//! completed update.
//!
//! The firmware implements a basic version of the contract. Besides the path,
//! it supports the `hold` and `asset` capabilities. It does not retry failed
//! attempts on its own: a failed attempt stays failed until the configuration
//! changes or another writer clears the failure count.

use core::fmt::Write as _;

use embassy_net::Stack;
use embedded_io_async::Read;
use embedded_io_async::Write;
use heapless::String;
use log::info;
use log::warn;
use sidex_embedded::types::Json;
use sidex_embedded::types::Seq;
use sidex_embedded::types::Str;

use super::api::devices::DeviceId;
use super::api::devices::GetDevicePropertyAction;
use super::api::devices::GetDevicePropertyOutput;
use super::api::devices::SetDevicePropertyAction;
use super::api::devices::SetDevicePropertyOutput;
use super::api::properties::OtaUpdateBlocked;
use super::api::properties::OtaUpdateBlockedCode;
use super::api::properties::OtaUpdateCapability;
use super::api::properties::OtaUpdateConfig;
use super::api::properties::OtaUpdateFailureCode;
use super::api::properties::OtaUpdateState;
use super::api::properties::OtaUpdateStatus;
use super::api::repositories::IssueAssetDownloadUrlAction;
use super::api::repositories::IssueAssetDownloadUrlOutput;
use super::api::repositories::PackageVersionId;
use super::api::repositories::ResolvePackageVersionAssetByPathAction;
use super::api::repositories::ResolvePackageVersionAssetByPathOutput;
use super::api::repositories::ResolvePackageVersionByPathAction;
use super::api::repositories::ResolvePackageVersionByPathOutput;
use super::config;
use super::download::Download;
use super::firmware::Firmware;
use super::hub;
use super::hub::Session;
use super::hub::WaitReadable;
use super::tls;

/// Property with the desired state.
const CONFIG_PROPERTY: &str = "dev.nexigon.ota.config";

/// Property with the actual state.
const STATUS_PROPERTY: &str = "dev.nexigon.ota.status";

/// Format version of the contract that the firmware implements.
const FORMAT_VERSION: u32 = 1;

/// Optional features of the contract that the firmware supports.
const CAPABILITIES: [OtaUpdateCapability<'static>; 2] =
    [OtaUpdateCapability::Hold, OtaUpdateCapability::Asset];

/// Asset installed from a version unless the configuration names another.
const DEFAULT_ASSET: &str = "firmware.bin";

/// Maximum size of a signature, which is about 72 bytes in DER format.
const MAX_SIGNATURE_SIZE: usize = 128;

/// ID of a package version or asset.
type Id = String<64>;

/// Error message for the status.
type Message = String<256>;

/// URL of a download, which carries a signature in its query.
type Url = String<1536>;

/// Result of a check.
pub enum Outcome {
    /// Nothing more to do until the next check.
    Done,
    /// An update has been written, and a reset activates it.
    Activate,
}

/// Checks for an update and installs it.
pub async fn check<S: Read + Write + WaitReadable>(
    session: &mut Session<'_, S>,
    device_id: &str,
    firmware: &mut Firmware,
    stack: Stack<'_>,
    buffers: &mut tls::Buffers,
) -> Result<Outcome, hub::Error> {
    let mut check = Check {
        session,
        device_id,
        status_version: None,
    };
    // The status may change concurrently, for example when a user clears the
    // failure count, in which case the check starts over once.
    for _ in 0..2 {
        match check.run(firmware, stack, buffers).await {
            Err(CheckError::Conflict) => {
                info!("the update status changed during the check, starting over");
            }
            Err(CheckError::Hub(error)) => return Err(error),
            Ok(outcome) => return Ok(outcome),
        }
    }
    Ok(Outcome::Done)
}

/// Error of a check.
enum CheckError {
    Hub(hub::Error),
    /// The status changed since it was read.
    Conflict,
}

impl From<hub::Error> for CheckError {
    fn from(error: hub::Error) -> Self {
        Self::Hub(error)
    }
}

/// One check, which reads the previous status and publishes a new one.
struct Check<'c, 's, S> {
    session: &'c mut Session<'s, S>,
    device_id: &'c str,
    /// Version of the status property, which writes compare and swap.
    status_version: Option<u64>,
}

impl<S: Read + Write + WaitReadable> Check<'_, '_, S> {
    async fn run(
        &mut self,
        firmware: &mut Firmware,
        stack: Stack<'_>,
        buffers: &mut tls::Buffers,
    ) -> Result<Outcome, CheckError> {
        let previous = self.previous_status().await?;
        let (config, config_version) = self.config().await?;
        let mut status = Status::new(config_version);

        // Firmware that the bootloader has just swapped in commits itself,
        // now that it has reached Nexigon Hub.
        if firmware.awaits_commit() {
            status.state = OtaUpdateState::Committing;
            status.target_version_id = previous.target_version_id.clone();
            status.next_version_id = previous.next_version_id.clone();
            self.publish(&status).await?;
        }
        if let Err(error) = firmware.commit() {
            warn!("failed to commit the firmware: {error}");
            let mut message = Message::new();
            let _ = write!(message, "failed to commit the firmware: {error}");
            return self.finish_failed(status, message).await;
        }

        let config = match config {
            Ok(config) => config,
            Err(ConfigError::UnsupportedFormat) => {
                status.state = OtaUpdateState::Blocked;
                status.blocked = Some(OtaUpdateBlockedCode::UnsupportedConfigFormat);
                return self.finish(&status).await;
            }
            Err(ConfigError::Invalid) => {
                return self
                    .finish_failed(status, message("invalid update configuration"))
                    .await;
            }
        };
        status.config = Some(config.clone());
        let Some(path) = config.path.as_deref() else {
            status.state = OtaUpdateState::Idle;
            return self.finish(&status).await;
        };
        let Some(path) = VersionPath::parse(path) else {
            return self
                .finish_failed(status, message("invalid version path"))
                .await;
        };
        let running_id = self.resolve(&path, crate::FIRMWARE_VERSION).await?;
        status.current_version_id = running_id.clone();

        // Find out how an attempt in progress before the reset ended.
        let mut completed = false;
        let mut failed = false;
        if let Some(state) = previous.state.filter(in_progress) {
            if running_id.is_some() && running_id == previous.next_version_id {
                info!("update completed");
                completed = true;
            } else {
                failed = true;
                let (code, error) = match state {
                    OtaUpdateState::Installing => (
                        OtaUpdateFailureCode::Interrupted,
                        "the installation was interrupted",
                    ),
                    _ => (
                        OtaUpdateFailureCode::RolledBack,
                        "the update was rolled back",
                    ),
                };
                warn!("update failed: {error}");
                status.failure_code = Some(code);
                status.last_error = Some(message(error));
            }
        }

        let Some(target_id) = self.resolve(&path, path.version).await? else {
            let mut error = Message::new();
            let _ = write!(error, "version {} does not exist", path.version);
            return self.finish_failed(status, error).await;
        };
        status.target_version_id = Some(target_id.clone());
        if running_id.as_ref() == Some(&target_id) {
            status.state = OtaUpdateState::Completed;
            status.failure_count = Some(0);
            status.failure_code = None;
            status.last_error = None;
            return self.finish(&status).await;
        }
        status.next_version_id = Some(target_id.clone());

        // The failure count belongs to the next version and the configuration.
        let same_attempt = previous.next_version_id.as_ref() == Some(&target_id)
            && previous.config.as_ref() == Some(&config);
        let mut failures = if same_attempt && !completed {
            previous.failure_count
        } else {
            0
        };
        if failed && same_attempt {
            failures += 1;
        } else if same_attempt && !completed {
            // The error of the last failed attempt stays until an attempt
            // succeeds.
            status.failure_code = previous.failure_code;
            status.last_error = previous.last_error.clone();
        }
        status.failure_count = Some(failures);

        if config.hold {
            status.state = OtaUpdateState::Blocked;
            status.blocked = Some(OtaUpdateBlockedCode::Hold);
            return self.finish(&status).await;
        }
        if failures > 0 {
            status.state = OtaUpdateState::Failed;
            return self.finish(&status).await;
        }

        let mut signature_asset = String::<68>::new();
        let _ = write!(signature_asset, "{}.sig", config.asset);
        let image = self.resolve_asset(&path, &target_id, &config.asset).await?;
        let signature = self
            .resolve_asset(&path, &target_id, &signature_asset)
            .await?;
        let (Some(image), Some(signature)) = (image, signature) else {
            status.state = OtaUpdateState::Blocked;
            status.blocked = Some(OtaUpdateBlockedCode::NoAsset);
            return self.finish(&status).await;
        };
        let image_url = self.download_url(&image).await?;
        let signature_url = self.download_url(&signature).await?;

        info!("installing version {}", path.version);
        status.state = OtaUpdateState::Installing;
        self.publish(&status).await?;
        let installed = install(
            self.session,
            firmware,
            stack,
            buffers,
            &image_url,
            &signature_url,
        )
        .await?;
        if let Err(error) = installed {
            warn!("update failed: {error}");
            status.failure_count = Some(failures + 1);
            status.failure_code = Some(OtaUpdateFailureCode::InstallFailed);
            return self.finish_failed(status, error).await;
        }
        info!("installed version {}, rebooting", path.version);
        status.state = OtaUpdateState::Rebooting;
        self.publish(&status).await?;
        Ok(Outcome::Activate)
    }

    /// Reads the previous status, which is the only state that persists across
    /// resets.
    async fn previous_status(&mut self) -> Result<Previous, CheckError> {
        let action = GetDevicePropertyAction {
            device_id: DeviceId(Str::new(self.device_id)),
            name: Str::new(STATUS_PROPERTY),
        };
        let output = self
            .session
            .invoke::<GetDevicePropertyOutput>("devices_GetProperty", &action)
            .await?;
        let mut previous = Previous::default();
        let GetDevicePropertyOutput::Found(property) = output else {
            self.status_version = None;
            return Ok(previous);
        };
        self.status_version = Some(property.version);
        // An unreadable status counts as none.
        let Ok(status) = property.value.decode::<OtaUpdateStatus>() else {
            return Ok(previous);
        };
        previous.state = status.state.and_then(known_state);
        previous.target_version_id = status.target_version_id.and_then(|id| copy(id.0));
        previous.next_version_id = status.next_version_id.and_then(|id| copy(id.0));
        previous.failure_count = status.failure_count.unwrap_or(0);
        previous.failure_code = status.failure_code.and_then(known_failure_code);
        previous.last_error = status.last_error.and_then(copy);
        previous.config = status
            .config
            .and_then(|config| Config::from_api(&config).ok());
        Ok(previous)
    }

    /// Reads the configuration and its version.
    async fn config(&mut self) -> Result<(Result<Config, ConfigError>, Option<u64>), CheckError> {
        let action = GetDevicePropertyAction {
            device_id: DeviceId(Str::new(self.device_id)),
            name: Str::new(CONFIG_PROPERTY),
        };
        let output = self
            .session
            .invoke::<GetDevicePropertyOutput>("devices_GetProperty", &action)
            .await?;
        Ok(match output {
            GetDevicePropertyOutput::NotFound => (Ok(Config::default()), None),
            GetDevicePropertyOutput::Found(property) => {
                let config = property
                    .value
                    .decode::<OtaUpdateConfig>()
                    .map_err(|_| ConfigError::Invalid)
                    .and_then(|config| Config::from_api(&config));
                (config, Some(property.version))
            }
        })
    }

    /// Resolves `version` in the package of `path`.
    async fn resolve(
        &mut self,
        path: &VersionPath<'_>,
        version: &str,
    ) -> Result<Option<Id>, CheckError> {
        let action = ResolvePackageVersionByPathAction {
            organization_id: None,
            repository: Str::new(path.repository),
            package: Str::new(path.package),
            tag: Str::new(version),
        };
        let output = self
            .session
            .invoke::<ResolvePackageVersionByPathOutput>(
                "repositories_ResolveVersionByPath",
                &action,
            )
            .await?;
        Ok(match output {
            ResolvePackageVersionByPathOutput::Found(found) => copy(found.version_id.0),
            ResolvePackageVersionByPathOutput::NotFound => None,
        })
    }

    /// Resolves the asset `filename` of a version.
    async fn resolve_asset(
        &mut self,
        path: &VersionPath<'_>,
        version_id: &str,
        filename: &str,
    ) -> Result<Option<Id>, CheckError> {
        let action = ResolvePackageVersionAssetByPathAction {
            organization_id: None,
            repository: Str::new(path.repository),
            package: Str::new(path.package),
            tag: Str::new(version_id),
            filename: Str::new(filename),
        };
        let output = self
            .session
            .invoke::<ResolvePackageVersionAssetByPathOutput>(
                "repositories_ResolveVersionAssetByPath",
                &action,
            )
            .await?;
        Ok(match output {
            ResolvePackageVersionAssetByPathOutput::Found(found) => copy(found.asset_id.0),
            ResolvePackageVersionAssetByPathOutput::NotFound => None,
        })
    }

    /// Issues a URL for downloading an asset.
    async fn download_url(&mut self, asset_id: &str) -> Result<Url, CheckError> {
        let action = IssueAssetDownloadUrlAction {
            asset_id: super::api::repositories::RepositoryAssetId(Str::new(asset_id)),
            filename: None,
        };
        let output = self
            .session
            .invoke::<IssueAssetDownloadUrlOutput>("repositories_IssueAssetDownloadUrl", &action)
            .await?;
        copy(output.url).ok_or(CheckError::Hub(hub::Error::Protocol(
            "download URL exceeds its buffer",
        )))
    }

    /// Publishes a failed status with `error`.
    async fn finish_failed(
        &mut self,
        mut status: Status,
        error: Message,
    ) -> Result<Outcome, CheckError> {
        status.state = OtaUpdateState::Failed;
        status.last_error = Some(error);
        self.finish(&status).await
    }

    /// Publishes the final status of the check.
    async fn finish(&mut self, status: &Status) -> Result<Outcome, CheckError> {
        self.publish(status).await?;
        Ok(Outcome::Done)
    }

    /// Publishes the status if it has not changed since it was read.
    async fn publish(&mut self, status: &Status) -> Result<(), CheckError> {
        let capabilities = Seq::new(&CAPABILITIES);
        let config = status.config.as_ref().map(Config::to_api);
        let blocked = status.blocked.map(|code| OtaUpdateBlocked {
            code,
            message: None,
        });
        let value = OtaUpdateStatus {
            format_version: Some(FORMAT_VERSION),
            capabilities: Some(capabilities),
            config,
            config_version: status.config_version,
            current_version: Some(Str::new(crate::FIRMWARE_VERSION)),
            current_version_id: status.current_version_id.as_deref().map(version_id),
            state: Some(status.state),
            blocked,
            target_version: None,
            target_version_id: status.target_version_id.as_deref().map(version_id),
            next_version_id: status.next_version_id.as_deref().map(version_id),
            failure_count: status.failure_count,
            failure_code: status.failure_code,
            last_error: status.last_error.as_deref().map(Str::new),
            next_attempt_at: None,
            updater: None,
        };
        let action = SetDevicePropertyAction {
            device_id: DeviceId(Str::new(self.device_id)),
            name: Str::new(STATUS_PROPERTY),
            value: Json::from_value(&value),
            protected: None,
            expected_version: self.status_version,
        };
        let output = self
            .session
            .invoke::<SetDevicePropertyOutput>("devices_SetProperty", &action)
            .await?;
        match output {
            SetDevicePropertyOutput::Set(set) => {
                self.status_version = set.version;
                Ok(())
            }
            SetDevicePropertyOutput::VersionConflict(_) | SetDevicePropertyOutput::Deleted => {
                Err(CheckError::Conflict)
            }
        }
    }
}

/// Downloads the image and its signature, writes the image to the DFU slot,
/// and marks it for the bootloader.
///
/// Returns an error message if the installation fails. Errors of the session
/// end the check.
async fn install<S: Read + Write + WaitReadable>(
    session: &mut Session<'_, S>,
    firmware: &mut Firmware,
    stack: Stack<'_>,
    buffers: &mut tls::Buffers,
    image_url: &str,
    signature_url: &str,
) -> Result<Result<(), Message>, hub::Error> {
    let mut signature = [0; MAX_SIGNATURE_SIZE];
    let signature_len = {
        let mut download = match Download::start(stack, buffers, signature_url).await {
            Ok(download) => download,
            Err(error) => return Ok(Err(describe("failed to download the signature", error))),
        };
        if download.len() > signature.len() {
            return Ok(Err(message("the signature is too large")));
        }
        let mut len = 0;
        while len < download.len() {
            match download.read(&mut signature[len..]).await {
                Ok(read) => len += read,
                Err(error) => return Ok(Err(describe("failed to download the signature", error))),
            }
        }
        len
    };
    session.poll().await?;

    let mut download = match Download::start(stack, buffers, image_url).await {
        Ok(download) => download,
        Err(error) => return Ok(Err(describe("failed to download the image", error))),
    };
    let total = download.len();
    info!("downloading {total} bytes");
    let mut update = match firmware.update(total) {
        Ok(update) => update,
        Err(error) => return Ok(Err(describe("failed to write the image", error))),
    };
    let mut chunk = [0; 4096];
    let mut written = 0;
    while written < total {
        // Fill the chunk, so that each write covers a whole page of flash.
        let mut len = 0;
        while len < chunk.len() && written + len < total {
            match download.read(&mut chunk[len..]).await {
                Ok(read) => len += read,
                Err(error) => return Ok(Err(describe("failed to download the image", error))),
            }
        }
        if let Err(error) = update.write(&chunk[..len]) {
            return Ok(Err(describe("failed to write the image", error)));
        }
        written += len;
        if written % (64 * 1024) == 0 || written == total {
            info!("downloaded {written} of {total} bytes");
        }
        session.poll().await?;
    }
    drop(download);
    if let Err(error) = update.finish(&signature[..signature_len]) {
        return Ok(Err(describe("failed to verify the image", error)));
    }
    Ok(Ok(()))
}

/// Indicates whether a state belongs to an attempt in progress.
fn in_progress(state: &OtaUpdateState<'_>) -> bool {
    matches!(
        state,
        OtaUpdateState::Installing
            | OtaUpdateState::Rebooting
            | OtaUpdateState::Committing
            | OtaUpdateState::Rollback
    )
}

/// Status that a check publishes.
struct Status {
    config: Option<Config>,
    config_version: Option<u64>,
    current_version_id: Option<Id>,
    state: OtaUpdateState<'static>,
    blocked: Option<OtaUpdateBlockedCode<'static>>,
    target_version_id: Option<Id>,
    next_version_id: Option<Id>,
    failure_count: Option<u32>,
    failure_code: Option<OtaUpdateFailureCode<'static>>,
    last_error: Option<Message>,
}

impl Status {
    fn new(config_version: Option<u64>) -> Self {
        Self {
            config: None,
            config_version,
            current_version_id: None,
            state: OtaUpdateState::Idle,
            blocked: None,
            target_version_id: None,
            next_version_id: None,
            failure_count: None,
            failure_code: None,
            last_error: None,
        }
    }
}

/// Parts of the previous status that a check uses.
#[derive(Default)]
struct Previous {
    state: Option<OtaUpdateState<'static>>,
    target_version_id: Option<Id>,
    next_version_id: Option<Id>,
    failure_count: u32,
    failure_code: Option<OtaUpdateFailureCode<'static>>,
    last_error: Option<Message>,
    config: Option<Config>,
}

/// Effective configuration, with the defaults of the firmware applied.
#[derive(Clone, PartialEq)]
struct Config {
    path: Option<String<128>>,
    asset: String<64>,
    hold: bool,
    rollout: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: config::UPDATE_PATH.and_then(|path| String::try_from(path).ok()),
            asset: String::try_from(DEFAULT_ASSET).expect("the default asset fits"),
            hold: false,
            rollout: None,
        }
    }
}

/// Error reading the configuration.
enum ConfigError {
    /// The configuration has a later format version.
    UnsupportedFormat,
    /// The configuration is invalid or does not fit into its buffers.
    Invalid,
}

impl Config {
    fn from_api(config: &OtaUpdateConfig<'_>) -> Result<Self, ConfigError> {
        if config
            .format_version
            .is_some_and(|version| version > FORMAT_VERSION)
        {
            return Err(ConfigError::UnsupportedFormat);
        }
        // Fields that the property sets replace the defaults. A simplification:
        // `null` keeps a default instead of removing it.
        let mut result = Self::default();
        if let Some(path) = config.path {
            result.path = Some(copy(path).ok_or(ConfigError::Invalid)?);
        }
        if let Some(asset) = config.asset {
            result.asset = copy(asset).ok_or(ConfigError::Invalid)?;
        }
        result.hold = config.hold.unwrap_or(false);
        result.rollout = config.rollout;
        Ok(result)
    }

    fn to_api(&self) -> OtaUpdateConfig<'_> {
        OtaUpdateConfig {
            format_version: None,
            path: self.path.as_deref().map(Str::new),
            asset: Some(Str::new(&self.asset)),
            hold: Some(self.hold),
            window: None,
            retry: None,
            spread: None,
            rollout: self.rollout,
        }
    }
}

/// Version path `<repository>/<package>/<version>`.
struct VersionPath<'p> {
    repository: &'p str,
    package: &'p str,
    version: &'p str,
}

impl<'p> VersionPath<'p> {
    fn parse(path: &'p str) -> Option<Self> {
        let mut parts = path.split('/');
        let path = Self {
            repository: parts.next()?,
            package: parts.next()?,
            version: parts.next()?,
        };
        parts.next().is_none().then_some(path)
    }
}

fn version_id(id: &str) -> PackageVersionId<'_> {
    PackageVersionId(Str::new(id))
}

/// Copies a decoded string, which borrows from the session, returning `None`
/// if it does not fit.
fn copy<const N: usize>(text: Str<'_>) -> Option<String<N>> {
    let mut string = String::new();
    write!(string, "{text}").ok()?;
    Some(string)
}

fn message(text: &str) -> Message {
    let mut message = Message::new();
    let _ = message.push_str(&text[..text.len().min(message.capacity())]);
    message
}

fn describe(context: &str, error: impl core::fmt::Display) -> Message {
    let mut message = Message::new();
    let _ = write!(message, "{context}: {error}");
    message
}

/// Returns a state that the firmware knows, which no longer borrows.
fn known_state(state: OtaUpdateState<'_>) -> Option<OtaUpdateState<'static>> {
    Some(match state {
        OtaUpdateState::Idle => OtaUpdateState::Idle,
        OtaUpdateState::Installing => OtaUpdateState::Installing,
        OtaUpdateState::Rebooting => OtaUpdateState::Rebooting,
        OtaUpdateState::Committing => OtaUpdateState::Committing,
        OtaUpdateState::Completed => OtaUpdateState::Completed,
        OtaUpdateState::Failed => OtaUpdateState::Failed,
        OtaUpdateState::Blocked => OtaUpdateState::Blocked,
        OtaUpdateState::Rollback => OtaUpdateState::Rollback,
        OtaUpdateState::Other(_) => return None,
    })
}

/// Returns a failure code that the firmware knows, which no longer borrows.
fn known_failure_code(code: OtaUpdateFailureCode<'_>) -> Option<OtaUpdateFailureCode<'static>> {
    Some(match code {
        OtaUpdateFailureCode::InstallFailed => OtaUpdateFailureCode::InstallFailed,
        OtaUpdateFailureCode::Interrupted => OtaUpdateFailureCode::Interrupted,
        OtaUpdateFailureCode::RebootFailed => OtaUpdateFailureCode::RebootFailed,
        OtaUpdateFailureCode::RolledBack => OtaUpdateFailureCode::RolledBack,
        OtaUpdateFailureCode::Other(_) => return None,
    })
}
