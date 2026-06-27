use embassy_time::{Duration, Timer};
use esp_hal::gpio::{AnyPin, Level, Output, OutputConfig};
use log::debug;
use max7219::MAX7219;

type LedMatrix =
    MAX7219<max7219::connectors::PinConnector<Output<'static>, Output<'static>, Output<'static>>>;

const DIGITS: [[u8; 5]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111], // 0
    [0b001, 0b001, 0b001, 0b001, 0b001], // 1
    [0b111, 0b001, 0b111, 0b100, 0b111], // 2
    [0b111, 0b001, 0b111, 0b001, 0b111], // 3
    [0b101, 0b101, 0b111, 0b001, 0b001], // 4
    [0b111, 0b100, 0b111, 0b001, 0b111], // 5
    [0b111, 0b100, 0b111, 0b101, 0b111], // 6
    [0b111, 0b001, 0b010, 0b010, 0b010], // 7
    [0b111, 0b101, 0b111, 0b101, 0b111], // 8
    [0b111, 0b101, 0b111, 0b001, 0b001], // 9
];

pub struct Display {
    led_matrix: LedMatrix,
}

impl Display {
    pub fn init(data: AnyPin<'static>, cs: AnyPin<'static>, sck: AnyPin<'static>) -> Self {
        let data = Output::new(data, Level::Low, OutputConfig::default());
        let cs = Output::new(cs, Level::High, OutputConfig::default());
        let sck = Output::new(sck, Level::Low, OutputConfig::default());

        let mut led_matrix = MAX7219::from_pins(1, data, cs, sck).unwrap();
        led_matrix.power_on().unwrap();
        led_matrix.set_intensity(0, 0x08).unwrap();

        debug!("Display initialized!\r\n");
        Display { led_matrix }
    }

    pub fn render(&mut self, frame: &[u8; 8]) {
        self.led_matrix.write_raw(0, frame).unwrap();
    }

    pub async fn show_score(&mut self, score: u8) {
        let tens = score / 10;
        let ones = score % 10;
        let has_tens = tens > 0;

        let mut frame = [0u8; 8];

        if has_tens {
            Display::draw_digit(&mut frame, tens as usize, 1, 2);
            Display::draw_digit(&mut frame, ones as usize, 5, 2);
        } else {
            Display::draw_digit(&mut frame, ones as usize, 5, 2);
        }

        self.render(&frame);

        Timer::after(Duration::from_millis(2000)).await;
    }

    fn draw_digit(frame: &mut [u8; 8], digit: usize, x_offset: u8, y_offset: u8) {
        for y in 0..5u8 {
            for x in 0..3u8 {
                if DIGITS[digit][y as usize] & (1 << (2 - x)) != 0 {
                    Display::turn_on_led(frame, x_offset + x, y_offset + (4 - y));
                }
            }
        }
    }

    pub async fn toggle_screen_on_and_off(&mut self) {
        for screen in 0..12 {
            if screen % 2 == 0 {
                let frame = [0xFF; 8];
                self.render(&frame);
            } else {
                let frame = [0x00; 8];
                self.render(&frame);
            }
            Timer::after(Duration::from_millis(100)).await;
        }
    }

    pub fn turn_on_led(frame: &mut [u8; 8], x: u8, y: u8) {
        let hardware_x = y;
        let hardware_y = 7 - x;

        frame[hardware_y as usize] |= 1 << hardware_x;
    }
}
