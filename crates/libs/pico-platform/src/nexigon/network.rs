//! Wi-Fi and the network stack through which the device reaches Nexigon Hub.
//!
//! [`start`] powers up the Wi-Fi chip of the Pico W, joins the network of the
//! [`Config`], and waits for an address from DHCP. The chip does not rejoin
//! the network on its own after losing it, so [`ensure_up`] joins it again.

use cyw43::JoinOptions;
use cyw43::aligned_bytes;
use cyw43_pio::DEFAULT_CLOCK_DIVIDER;
use cyw43_pio::PioSpi;
use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_net::StackResources;
use embassy_rp::Peri;
use embassy_rp::bind_interrupts;
use embassy_rp::clocks::RoscRng;
use embassy_rp::dma;
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_rp::peripherals::DMA_CH0;
use embassy_rp::peripherals::PIN_23;
use embassy_rp::peripherals::PIN_24;
use embassy_rp::peripherals::PIN_25;
use embassy_rp::peripherals::PIN_29;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio;
use embassy_rp::pio::Pio;
use embassy_time::Duration;
use embassy_time::Timer;
use embassy_time::with_timeout;
use log::info;
use log::warn;
use static_cell::StaticCell;

use super::config::Config;

/// Delay before joining the network again after a failure.
const RETRY_DELAY: Duration = Duration::from_secs(10);

/// Time within which DHCP must provide an address after joining the network.
const DHCP_TIMEOUT: Duration = Duration::from_secs(30);

/// Number of sockets that the application may use at a time.
pub const APPLICATION_SOCKETS: usize = 2;

/// Number of sockets of the network stack: DHCP and DNS, which the stack
/// keeps open, the connection to Nexigon Hub, the connection of a download,
/// SNTP, and the sockets of the application.
const SOCKETS: usize = 5 + APPLICATION_SOCKETS;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

/// Peripherals through which the RP2040 of a Pico W drives its Wi-Fi chip.
pub struct WifiPeripherals {
    /// Power of the chip.
    pub power: Peri<'static, PIN_23>,
    /// Data line of the chip.
    pub data: Peri<'static, PIN_24>,
    /// Chip select of the chip.
    pub chip_select: Peri<'static, PIN_25>,
    /// Clock of the chip.
    pub clock: Peri<'static, PIN_29>,
    /// PIO block that drives the bus of the chip.
    pub pio: Peri<'static, PIO0>,
    /// DMA channel of the bus of the chip.
    pub dma: Peri<'static, DMA_CH0>,
}

/// Starts the Wi-Fi chip, joins the network, and returns the network stack
/// and the control of the chip, which also drives the LED of the Pico W.
pub async fn start(
    spawner: Spawner,
    peripherals: WifiPeripherals,
    config: &'static Config,
) -> (Stack<'static>, cyw43::Control<'static>) {
    let WifiPeripherals {
        power,
        data,
        chip_select,
        clock,
        pio,
        dma,
    } = peripherals;
    let wifi_firmware = aligned_bytes!("../../firmware/cyw43/43439A0.bin");
    let clm = aligned_bytes!("../../firmware/cyw43/43439A0_clm.bin");
    let nvram = aligned_bytes!("../../firmware/cyw43/nvram_rp2040.bin");

    let power = Output::new(power, Level::Low);
    let chip_select = Output::new(chip_select, Level::High);
    let mut pio = Pio::new(pio, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        chip_select,
        data,
        clock,
        dma::Channel::new(dma, Irqs),
    );

    static WIFI_STATE: StaticCell<cyw43::State> = StaticCell::new();
    let (net_device, mut control, runner) = cyw43::new(
        WIFI_STATE.init(cyw43::State::new()),
        power,
        spi,
        wifi_firmware,
        nvram,
    )
    .await;
    spawner.spawn(wifi_task(runner).unwrap());
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    static NET_RESOURCES: StaticCell<StackResources<SOCKETS>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        embassy_net::Config::dhcpv4(Default::default()),
        NET_RESOURCES.init(StackResources::new()),
        RoscRng.next_u64(),
    );
    spawner.spawn(net_task(runner).unwrap());

    join(stack, &mut control, config).await;
    (stack, control)
}

/// Joins the network again if the device has lost its address, for instance,
/// because the access point restarted.
pub async fn ensure_up(
    stack: Stack<'static>,
    control: &mut cyw43::Control<'static>,
    config: &'static Config,
) {
    if !stack.is_config_up() {
        warn!("lost the Wi-Fi network");
        control.leave().await;
        join(stack, control, config).await;
    }
}

/// Joins the network and waits for an address from DHCP, retrying until both
/// succeed.
async fn join(
    stack: Stack<'static>,
    control: &mut cyw43::Control<'static>,
    config: &'static Config,
) {
    loop {
        info!("joining Wi-Fi network {}", config.wifi_ssid);
        let options = JoinOptions::new(config.wifi_password.as_bytes());
        match control.join(config.wifi_ssid, options).await {
            Ok(()) => match with_timeout(DHCP_TIMEOUT, stack.wait_config_up()).await {
                Ok(()) => {
                    info!(
                        "network is up: {:?}",
                        stack.config_v4().map(|config| config.address)
                    );
                    return;
                }
                Err(_) => {
                    warn!("received no address from DHCP");
                    control.leave().await;
                }
            },
            Err(error) => warn!("failed to join Wi-Fi network: {error:?}"),
        }
        Timer::after(RETRY_DELAY).await;
    }
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>) -> ! {
    runner.run().await
}
