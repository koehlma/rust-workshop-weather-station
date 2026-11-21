//! Wi-Fi and the network stack through which the device reaches Nexigon Hub.
//!
//! [`start`] powers up the Wi-Fi chip of the Pico W, joins the network from
//! [`config`], and waits for an address from DHCP.

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
use log::info;
use log::warn;
use static_cell::StaticCell;

use super::config;

/// Delay before joining the network again after a failure.
const RETRY_DELAY: Duration = Duration::from_secs(10);

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

/// Starts the Wi-Fi chip, joins the network, and returns the network stack
/// and the control of the chip, which also drives the LED of the Pico W.
pub async fn start(
    spawner: Spawner,
    power: Peri<'static, PIN_23>,
    data: Peri<'static, PIN_24>,
    chip_select: Peri<'static, PIN_25>,
    clock: Peri<'static, PIN_29>,
    pio: Peri<'static, PIO0>,
    dma: Peri<'static, DMA_CH0>,
) -> (Stack<'static>, cyw43::Control<'static>) {
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

    static NET_RESOURCES: StaticCell<StackResources<4>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        embassy_net::Config::dhcpv4(Default::default()),
        NET_RESOURCES.init(StackResources::new()),
        RoscRng.next_u64(),
    );
    spawner.spawn(net_task(runner).unwrap());

    info!("joining Wi-Fi network {}", config::WIFI_SSID);
    while let Err(error) = control
        .join(
            config::WIFI_SSID,
            JoinOptions::new(config::WIFI_PASSWORD.as_bytes()),
        )
        .await
    {
        warn!("failed to join Wi-Fi network: {error:?}");
        Timer::after(RETRY_DELAY).await;
    }
    stack.wait_config_up().await;
    info!(
        "network is up: {:?}",
        stack.config_v4().map(|config| config.address)
    );
    (stack, control)
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
