use esp_hal::{Blocking, delay::Delay, gpio::Output, spi::master::Spi};

pub struct Max7219<'a> {
    spi: Spi<'a, Blocking>,
    cs: Output<'a>,
}

impl<'a> Max7219<'a> {
    pub fn new(spi: Spi<'a, Blocking>, cs: Output<'a>, delay: &mut Delay) -> Self {
        let mut dev = Self { spi, cs };

        dev.cs.set_high();

        // MAX7219 init sequence
        dev.write_register(0x0F, 0x00); // display test off
        dev.write_register(0x0C, 0x01); // shutdown off
        dev.write_register(0x0B, 0x07); // scan limit = 8 digits
        dev.write_register(0x09, 0x00); // decode mode off
        dev.write_register(0x0A, 0x08); // intensity

        dev.clear();

        delay.delay_millis(10);

        dev
    }

    fn write_register(&mut self, reg: u8, data: u8) {
        self.cs.set_low();

        let buf = [reg, data];
        self.spi.write(&buf).unwrap();

        self.cs.set_high();
    }

    pub fn clear(&mut self) {
        for row in 1..=8 {
            self.write_register(row, 0x00);
        }
    }

    /// row: 0-7
    /// col: 0-7
    pub fn set_pixel(&mut self, row: u8, col: u8) {
        let value = 1 << col;

        // MAX7219 rows are 1-indexed
        self.write_register(row + 1, value);
    }
}
