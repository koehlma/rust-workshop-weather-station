//! Commands that Nexigon Hub can ask the device to run.
//!
//! The device describes its commands in the `dev.nexigon.commands` property,
//! from which Nexigon Hub offers them in its UI.

use embassy_time::Instant;
use embedded_io_async::Read;
use embedded_io_async::Write;
use log::info;
use sidex_embedded::Decode;
use sidex_embedded::DecodeError;
use sidex_embedded::Encode;
use sidex_embedded::EncodeError;
use sidex_embedded::json::Reader;
use sidex_embedded::json::Writer;
use sidex_embedded::types::Json;
use sidex_embedded::types::Seq;
use sidex_embedded::types::Str;

use super::api::devices::DeviceCommandDoneData;
use super::api::devices::DeviceCommandInvokeData;
use super::api::devices::DeviceCommandStatus;
use super::api::devices::DeviceId;
use super::api::devices::SetDevicePropertyAction;
use super::api::devices::SetDevicePropertyOutput;
use super::api::properties::DeviceCommandDescriptor;
use super::api::properties::DeviceCommandManifest;
use super::hub;
use super::hub::Session;
use super::hub::WaitReadable;

/// JSON Schema of the input of `set-led`.
const SET_LED_INPUT: &str = r#"{
    "type": "object",
    "properties": {
        "on": { "type": "boolean", "description": "Whether to turn the LED on." }
    },
    "required": ["on"],
    "additionalProperties": false
}"#;

/// JSON Schema of the output of `get-status`.
const GET_STATUS_OUTPUT: &str = r#"{
    "type": "object",
    "properties": {
        "uptimeSeconds": { "type": "integer", "description": "Seconds since the device started." },
        "led": { "type": "boolean", "description": "Whether the LED is on." }
    },
    "required": ["uptimeSeconds", "led"]
}"#;

/// Command that requests an immediate update check, as in Nexigon Agent.
const CHECK_UPDATE_COMMAND: &str = "nexigon.ota.check";

/// State of the device that commands act on.
pub struct Device {
    led: bool,
    update_check_requested: bool,
}

impl Device {
    /// Creates the state of a device whose LED is off.
    pub fn new() -> Self {
        Self {
            led: false,
            update_check_requested: false,
        }
    }

    /// Returns whether a command has requested an update check since the
    /// last call.
    pub fn take_update_check_request(&mut self) -> bool {
        core::mem::take(&mut self.update_check_requested)
    }

    /// Turns the LED on or off through the Wi-Fi chip, which drives it.
    async fn set_led(&mut self, control: &mut cyw43::Control<'static>, on: bool) {
        control.gpio_set(0, on).await;
        self.led = on;
    }
}

/// Publishes the descriptions of the commands.
pub async fn publish_manifest<S: Read + Write + WaitReadable>(
    session: &mut Session<'_, S>,
    device_id: &str,
) -> Result<(), hub::Error> {
    let commands = [
        DeviceCommandDescriptor {
            name: Str::new("set-led"),
            description: Some(Str::new("Turn the LED of the board on or off.")),
            category: None,
            input: Some(Json::parse(SET_LED_INPUT).expect("the schema is valid JSON")),
            output: None,
        },
        DeviceCommandDescriptor {
            name: Str::new("get-status"),
            description: Some(Str::new("Get the uptime and the state of the LED.")),
            category: None,
            input: None,
            output: Some(Json::parse(GET_STATUS_OUTPUT).expect("the schema is valid JSON")),
        },
        DeviceCommandDescriptor {
            name: Str::new(CHECK_UPDATE_COMMAND),
            description: Some(Str::new("Request an immediate update check.")),
            category: Some(Str::new("OTA")),
            input: None,
            output: None,
        },
    ];
    let manifest = DeviceCommandManifest {
        commands: Seq::new(&commands),
    };
    let action = SetDevicePropertyAction {
        device_id: DeviceId(Str::new(device_id)),
        name: Str::new("dev.nexigon.commands"),
        value: Json::from_value(&manifest),
        protected: None,
        expected_version: None,
    };
    session
        .invoke::<SetDevicePropertyOutput>("devices_SetProperty", &action)
        .await?;
    Ok(())
}

/// Runs the command that Nexigon Hub asked for and answers it.
pub async fn run<S: Read + Write + WaitReadable>(
    session: &mut Session<'_, S>,
    device: &mut Device,
    control: &mut cyw43::Control<'static>,
) -> Result<(), hub::Error> {
    let started = Instant::now();
    let result = match session.command() {
        Ok(invocation) => {
            info!("running command {}", invocation.command);
            execute(&invocation, device, control).await
        }
        Err(error) => Err(error),
    };
    let duration_ms = started.elapsed().as_millis();
    let done = match &result {
        Ok(output) => DeviceCommandDoneData {
            status: DeviceCommandStatus::Ok,
            output: output.as_ref().map(|output| Json::from_value(output)),
            error: None,
            log_tail: Seq::new(&[]),
            duration_ms,
        },
        Err(error) => DeviceCommandDoneData {
            status: DeviceCommandStatus::Error,
            output: None,
            error: Some(Str::new(error)),
            log_tail: Seq::new(&[]),
            duration_ms,
        },
    };
    session.complete(&done).await
}

/// Executes a command, returning its output or an error message.
async fn execute(
    invocation: &DeviceCommandInvokeData<'_>,
    device: &mut Device,
    control: &mut cyw43::Control<'static>,
) -> Result<Option<Status>, &'static str> {
    match invocation.command.as_str() {
        Some("set-led") => {
            let input = invocation
                .input
                .decode::<SetLedInput>()
                .map_err(|_| "the input is invalid")?;
            device.set_led(control, input.on).await;
            Ok(None)
        }
        Some("get-status") => Ok(Some(Status {
            uptime_seconds: Instant::now().as_secs(),
            led: device.led,
        })),
        Some(CHECK_UPDATE_COMMAND) => {
            device.update_check_requested = true;
            Ok(None)
        }
        _ => Err("unknown command"),
    }
}

/// Input of `set-led`.
struct SetLedInput {
    on: bool,
}

impl<'a> Decode<'a> for SetLedInput {
    const IS_RECORD: bool = true;

    fn decode(reader: &mut Reader<'a>) -> Result<Self, DecodeError> {
        let fields = reader.record(&[&["on"]])?;
        Ok(Self {
            on: fields.required(0)?,
        })
    }
}

/// Output of `get-status`.
struct Status {
    uptime_seconds: u64,
    led: bool,
}

impl Encode for Status {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), EncodeError> {
        let mut object = writer.object()?;
        self.uptime_seconds.encode(object.field("uptimeSeconds")?)?;
        self.led.encode(object.field("led")?)?;
        object.end()
    }
}
