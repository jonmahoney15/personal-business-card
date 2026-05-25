#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, delay::Delay};
use log::{error, info};

#[path = "../max7219.rs"]
mod max7219;
use max7219::Max7219;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let mut delay = Delay::new();

    // SPI pins
    let sck = peripherals.GPIO5;
    let mosi = peripherals.GPIO4;
    let cs = Output::new(peripherals.GPIO6, Level::High, OutputConfig::default());

    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_mhz(1))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sck)
    .with_mosi(mosi);

    let btn_up = Input::new(peripherals.GPIO8, InputConfig::default().with_pull(Pull::Up));
    let btn_down = Input::new(peripherals.GPIO3, InputConfig::default().with_pull(Pull::Up));
    let btn_left = Input::new(peripherals.GPIO2, InputConfig::default().with_pull(Pull::Up));
    let btn_right = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));

    let mut matrix = Max7219::new(spi, cs, &mut delay);
    matrix.clear();

    let mut row = 3;
    let mut col = 3;
    let mut dr = 0;
    let mut dc = 1;

    loop {
        if btn_up.is_low() {
            (dr, dc) = (-1, 0);
        }
        if btn_down.is_low() {
            (dr, dc) = (1, 0);
        }
        if btn_left.is_low() {
            (dr, dc) = (0, -1);
        }
        if btn_right.is_low() {
            (dr, dc) = (0, 1);
        }

        row = ((row as i8 + dr).rem_euclid(8)) as u8;
        col = ((col as i8 + dc).rem_euclid(8)) as u8;

        matrix.clear();
        matrix.set_pixel(row, col);

        Timer::after(Duration::from_millis(150)).await;
    }
}
