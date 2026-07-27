#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

mod display;
mod game;
mod snake;

use display::Display;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{AnyPin, Input, InputConfig, Pull};
use esp_hal::timer::timg::TimerGroup;
use game::Game;
use log::{debug, error};
use snake::Direction;
#[cfg(debug_assertions)]
use {
    embedded_io_async::Read,
    esp_hal::usb_serial_jtag::{UsbSerialJtag, UsbSerialJtagRx},
};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

static DIR_CHANNEL: Channel<CriticalSectionRawMutex, Direction, 8> = Channel::new();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    initialize_embassy(peripherals.TIMG0, peripherals.SW_INTERRUPT);

    let mut display = Display::init(
        peripherals.GPIO4.into(),
        peripherals.GPIO6.into(),
        peripherals.GPIO5.into(),
    );

    #[cfg(debug_assertions)]
    {
        let usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE).into_async();
        let (rx, _tx) = usb_serial.split();
        spawner.spawn(keyboard_input(rx).expect("spawn keyboard input"));
    }

    spawner.spawn(button(peripherals.GPIO8.into(), Direction::Up).expect("spawn up button"));
    spawner.spawn(button(peripherals.GPIO3.into(), Direction::Down).expect("spawn down button"));
    spawner.spawn(button(peripherals.GPIO2.into(), Direction::Left).expect("spawn left button"));
    spawner.spawn(button(peripherals.GPIO9.into(), Direction::Right).expect("spawn right button"));

    let mut game = Game::new();

    display.render(&game.frame());

    loop {
        if let Ok(direction) = DIR_CHANNEL.try_receive() {
            game.change_direction(direction);
        }

        game.tick();

        display.render(&game.frame());

        if !game.is_running() {
            debug!("Game over! Score: {}\r\n", game.score);
            display.show_score(game.score).await;
            display.toggle_screen_on_and_off().await;
            game.reset();
        }

        Timer::after(Duration::from_millis(300)).await;
    }
}

#[embassy_executor::task(pool_size = 4)]
async fn button(pin: AnyPin<'static>, dir: Direction) {
    let config = InputConfig::default().with_pull(Pull::Up);
    let btn = Input::new(pin, config);
    loop {
        if btn.is_low() {
            DIR_CHANNEL.send(dir).await;
            debug!("Button press {:?}\r", dir);
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

fn initialize_embassy(
    timg0: esp_hal::peripherals::TIMG0<'static>,
    sw_interrupt: esp_hal::peripherals::SW_INTERRUPT<'static>,
) {
    let timg0 = TimerGroup::new(timg0);
    let sw_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(sw_interrupt);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    debug!("Embassy initialized!\r\n");
}

#[cfg(debug_assertions)]
#[embassy_executor::task]
async fn keyboard_input(mut rx: UsbSerialJtagRx<'static, esp_hal::Async>) {
    debug!("Keyboard input enabled (HJKL vim keys)\r\n");
    let mut buffer = [0u8; 1];
    loop {
        if let Ok(1) = rx.read(&mut buffer).await {
            let direction = match buffer[0] {
                b'h' | b'H' => Some(Direction::Left),
                b'j' | b'J' => Some(Direction::Down),
                b'k' | b'K' => Some(Direction::Up),
                b'l' | b'L' => Some(Direction::Right),
                _ => None,
            };

            if let Some(direction) = direction {
                DIR_CHANNEL.send(direction).await;
                debug!("Keyboard {:?}\r", direction);
            }
        }
    }
}
