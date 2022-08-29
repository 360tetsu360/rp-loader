//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use atm0130::Color;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use fugit::RateExtU32;
use panic_probe as _;

use rp_pico as bsp;
use rp_pico::entry;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    self,
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use embedded_sdmmc::filesystem::Mode;
use embedded_sdmmc::{Controller, SdMmcSpi, TimeSource, Timestamp, VolumeIdx};

mod artemis;
mod atm0130;

#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    // In theory you could use the RTC of the rp2040 here, if you had
    // any external time synchronizing device.
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    // Initialize display
    let _atm0130_sclk = pins.gpio2.into_mode::<hal::gpio::FunctionSpi>();
    let _atm0130_mosi = pins.gpio3.into_mode::<hal::gpio::FunctionSpi>();
    let _atm0130_miso = pins.gpio4.into_mode::<hal::gpio::FunctionSpi>();
    let spi = hal::Spi::<_, _, 8>::new(pac.SPI0);
    let ss = pins.gpio5.into_push_pull_output();
    let dc = pins.gpio14.into_push_pull_output();
    let res = pins.gpio15.into_push_pull_output();

    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        &embedded_hal::spi::MODE_0,
    );

    let mut display = atm0130::Atm0130::init(spi, ss, dc, res);

    display.begin(&mut delay);

    let black = Color(0, 0, 0);
    display.draw_rect(0, 0, 240, 240, black);

    display.draw_logo(120 - artemis::IMG_WIDTH / 2, 120 - artemis::IMG_HEIGHT / 2);

    delay.delay_ms(1000);

    display.draw_rect(0, 0, 240, 240, Color(0, 0, 0));
    display.draw_info("Reading SD card.");

    // Initialize sd card
    let _sd_sclk = pins.gpio10.into_mode::<hal::gpio::FunctionSpi>();
    let _sd_mosi = pins.gpio11.into_mode::<hal::gpio::FunctionSpi>();
    let _sd_miso = pins.gpio12.into_mode::<hal::gpio::FunctionSpi>();
    let cs = pins.gpio13.into_push_pull_output();
    let spi = hal::Spi::<_, _, 8>::new(pac.SPI1);

    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        20.MHz(),
        &embedded_hal::spi::MODE_0,
    );

    let mut sdspi = SdMmcSpi::new(spi, cs);
    let block = sdspi.acquire().unwrap();
    let mut controller = Controller::new(block, DummyTimesource::default());

    let mut volume = controller.get_volume(VolumeIdx(0)).unwrap();
    let dir = controller.open_root_dir(&volume).unwrap();

    display.draw_rect(0, 0, 240, 240, Color(0, 0, 0));

    if let Ok(mut file) =
        controller.open_file_in_dir(&mut volume, &dir, "README.TXT", Mode::ReadOnly)
    {
        let mut buf = [0u8; 32];
        let read_count = controller.read(&volume, &mut file, &mut buf).unwrap();

        controller.close_file(&volume, file).unwrap();

        if read_count >= 2 {
            let text = unsafe { core::str::from_utf8_unchecked_mut(&mut buf[..read_count]) };
            led_pin.set_low().unwrap();

            display.draw_rect(0, 0, 240, 240, black);
            let size = 1;

            let text_size = atm0130::text_size(text, size);
            let x = 120 - text_size.0 / 2;
            let y = 120 - text_size.1 / 2;
            display.draw_rect(x, y, text_size.0, text_size.1, black);
            display.draw_text(text, x, y, size, Color(255, 255, 255), black);
        }
    }
    delay.delay_ms(2000);

    loop {
        cortex_m::asm::wfi();
    }
}

// End of file
