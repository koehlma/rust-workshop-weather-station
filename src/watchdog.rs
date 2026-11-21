//! Watchdog, which resets the device when the firmware hangs.
//!
//! The bootloader starts the watchdog, so the firmware must feed it from the
//! start. [`task`] feeds it while the executor runs. With the `nexigon`
//! feature, `stop_feeding` lets it reset the device on purpose to roll back an
//! update.

use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use embassy_rp::watchdog::Watchdog;
use embassy_time::Duration;
use embassy_time::Timer;

/// Time after which the watchdog resets the device unless it is fed.
const TIMEOUT: Duration = Duration::from_secs(8);

/// Interval at which [`task`] feeds the watchdog.
const FEED_INTERVAL: Duration = Duration::from_secs(1);

/// Indicates whether [`task`] feeds the watchdog.
static FEEDING: AtomicBool = AtomicBool::new(true);

/// Feeds the watchdog until `stop_feeding` is called.
#[embassy_executor::task]
pub async fn task(mut watchdog: Watchdog) -> ! {
    watchdog.start(TIMEOUT);
    loop {
        if FEEDING.load(Ordering::Relaxed) {
            watchdog.feed(TIMEOUT);
        }
        Timer::after(FEED_INTERVAL).await;
    }
}

/// Stops feeding the watchdog, which then resets the device.
#[cfg(feature = "nexigon")]
pub fn stop_feeding() {
    FEEDING.store(false, Ordering::Relaxed);
}
