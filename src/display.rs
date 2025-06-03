use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};
use ssd1306::{
    prelude::*,
    I2CDisplayInterface,
    Ssd1306,
    mode::BufferedGraphicsMode,
};

pub struct Display {
    display: Ssd1306<I2CInterface<I2cDriver<'static>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
}

impl Display {
    pub fn new(i2c: I2cDriver<'static>) -> Self {
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();
        display.clear_buffer();
        display.flush().unwrap();

        Self { display }
    }

    pub fn show_text(&mut self, text: &str, x: i32, y: i32) -> Result<(), ()> {
        let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        Text::new(text, Point::new(x, y), style)
            .draw(&mut self.display)
            .map_err(|_| ())?;
        self.display.flush().map_err(|_| ())
    }

    pub fn clear(&mut self) -> Result<(), ()> {
        self.display.clear_buffer();
        self.display.flush().map_err(|_| ())
    }
}

pub fn init_display() -> Result<Display, esp_idf_hal::i2c::I2cError> {
    let peripherals = esp_idf_hal::peripherals::Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio8;
    let scl = peripherals.pins.gpio9;
    
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &I2cConfig::new().baudrate(Hertz(400_000)),
    )?;

    Ok(Display::new(i2c))
} 