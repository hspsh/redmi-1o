mod qr_generator;

use embedded_graphics::{
    image::{Image, ImageRawLE}, // not needed for just text
    mono_font::{ascii::FONT_6X10, ascii::FONT_6X13_BOLD, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    i2c::{I2cConfig, I2cDriver},
    io::Error,
    prelude::*,
};

use ssd1306_i2c::{prelude::*, Builder}; // was use sh1106:: ...

const fn hex_to_bytes(hex: &str) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let mut i = 0;
    let mut j = 0;
    while i < hex.len() && j < 32 {
        let byte = match hex.as_bytes()[i] {
            b'0'..=b'9' => hex.as_bytes()[i] - b'0',
            b'a'..=b'f' => hex.as_bytes()[i] - b'a' + 10,
            b'A'..=b'F' => hex.as_bytes()[i] - b'A' + 10,
            _ => 0,
        };
        if i % 2 == 0 {
            bytes[j] = byte << 4;
        } else {
            bytes[j] |= byte;
            j += 1;
        }
        i += 1;
    }
    bytes
}

static SECRET_HEX: [u8; 32] = hex_to_bytes(env!("SECRET_HEX")); // paySecret
static USERID: &str = env!("USER_ID"); // your żappka user id/ployId

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Trace); // Set to Trace to see all possible logs

    // Configure ESP-IDF log level
    // unsafe {
    //     esp_idf_sys::esp_log_level_set("*", esp_idf_sys::esp_log_level_t_ESP_LOG_VERBOSE);
    // }

    // Set the log level for the ssd1306-i2c crate

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let pins = peripherals.pins;
    let sda = pins.gpio5; // esp32-c3  has pins.gpio0;
    let scl = pins.gpio6; // esp32-c3  haspins.gpio1;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into()); // works ok at 400 kHz with short bus length
    let mut i2c_dev = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    //creating i2c_dev
    log::info!("Scanning I2C bus...");
    for addr in 0..128 {
        if i2c_dev.write(addr, &[0], 100).is_ok() {
            log::info!("Found device at address: 0x{:02x}", addr);
        }
    }
    FreeRtos::delay_ms(100);

    // create and ssd1306-i2c instance using builder
    let mut display: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x64NoOffset)
        .with_i2c_addr(0x3c) // your LCD may used 0x3c the primary address
        .with_rotation(DisplayRotation::Rotate0)
        .connect_i2c(i2c_dev)
        .into();

    // Add this after

    log::info!("calling display.init()");
    FreeRtos::delay_ms(100);
    display.init().unwrap();

    FreeRtos::delay_ms(100);

    log::info!("calling display.flush()");
    display.flush().unwrap();

    log::info!("calling display.clear()");
    display.clear();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let text_style_bold = MonoTextStyleBuilder::new()
        .font(&FONT_6X13_BOLD)
        .text_color(BinaryColor::On)
        .build();

    log::info!("displaying Hello world! on LCD-OLED");
    Text::with_baseline(
        "Zappka ESP",
        Point::new(32,16),
        text_style_bold,
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();
    log::info!("displaying Hello Rust! on LCD-OLED");
    Text::with_baseline("SSD1306-I2C", Point::new(32, 39), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    //delay
    FreeRtos::delay_ms(1000);

    
    display.flush().unwrap();

    qr_generator::run_qr_generator(&SECRET_HEX, USERID);
}
