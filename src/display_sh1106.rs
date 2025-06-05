use embedded_graphics::{
    image::{Image, ImageRawLE},
    mono_font::{
        ascii::FONT_6X10, ascii::FONT_6X13_BOLD, MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use esp_idf_hal::{delay::FreeRtos, i2c::I2cDriver};

use sh1106::{prelude::*, Builder};

const COMPILE_TIME: &str = env!("COMPILE_TIME");

pub struct Display {
    display: GraphicsMode<I2cInterface<I2cDriver<'static>>>,
}

impl Display {
    pub fn new(
        i2c_dev: I2cDriver<'static>,
    ) -> Result<Self, ssd1306_i2c::Error> {
        let mut display: GraphicsMode<_> = Builder::new()
            .with_size(DisplaySize::Display128x128)
            .with_i2c_addr(0x3c)
            .with_rotation(DisplayRotation::Rotate0)
            .connect_i2c(i2c_dev)
            .into();

        log::info!("Initializing display...");
        FreeRtos::delay_ms(100);
        display.init();
        FreeRtos::delay_ms(100);
        display.flush();
        display.clear();

        Ok(Self { display })
    }

    pub fn show_welcome_screen(&mut self) -> Result<(), ssd1306_i2c::Error> {
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let text_style_bold = MonoTextStyleBuilder::new()
            .font(&FONT_6X13_BOLD)
            .text_color(BinaryColor::On)
            .build();

        log::info!("Displaying welcome screen");
        Text::with_baseline(
            "Zappka ESP",
            Point::new(32, 16),
            text_style_bold,
            Baseline::Top,
        )
        .draw(&mut self.display)?;

        Text::with_baseline(
            COMPILE_TIME,
            Point::new(32, 39),
            text_style,
            Baseline::Top,
        )
        .draw(&mut self.display)?;

        self.display.flush();
        FreeRtos::delay_ms(1000);

        self.display.clear();
        self.display.flush();

        Ok(())
    }

    pub fn draw_qr_by_str(
        &mut self,
        qr_str: &str,
    ) -> Result<(), ssd1306_i2c::Error> {
        let mut buf = [0u8; 128];

        // Process each line of the QR code string
        for (y, line) in qr_str.lines().take(32).enumerate() {
            for (x, ch) in line.chars().take(32).enumerate() {
                if ch == '.' {
                    // Set bit at position x in the byte for row y
                    buf[y * 4 + (x / 8)] |= 1 << (7 - (x % 8));
                }
            }
        }

        let qr_im: ImageRawLE<BinaryColor> = ImageRawLE::new(&buf, 32);

        

        Image::new(&qr_im, Point::new(5, 5))
            .draw(&mut self.display)
            .unwrap();

        self.display.flush();
        Ok(())
    }

    // pub fn clear(&mut self) -> Result<(), ssd1306_i2c::Error> {
    //     self.display.clear();
    //     self.display.flush()?;
    //     Ok(())
    // }

    // pub fn flush(&mut self) -> Result<(), ssd1306_i2c::Error> {
    //     self.display.flush()?;
    //     Ok(())
    // }
}

// pub fn scan_i2c_bus(i2c_dev: &mut I2cDriver) {
//     log::info!("Scanning I2C bus...");
//     for addr in 0..128 {
//         if i2c_dev.write(addr, &[0], 100).is_ok() {
//             log::info!("Found device at address: 0x{:02x}", addr);
//         }
//     }
//     FreeRtos::delay_ms(100);
// }
