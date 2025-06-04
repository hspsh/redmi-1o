mod qr_generator;
mod display;

use esp_idf_hal::{
    delay::{Ets, FreeRtos},
    i2c::{I2cConfig, I2cDriver},
    io::Error,
    prelude::*,
};

use crate::qr_generator::get_qr_str;

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
    log::set_max_level(log::LevelFilter::Trace);

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    let pins = peripherals.pins;
    let sda = pins.gpio5;
    let scl = pins.gpio6;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_dev = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    // Scan I2C bus
    // display::Display::scan_i2c_bus(&mut i2c_dev);

    // Initialize display
    let mut display = display::Display::new(i2c_dev).unwrap();
    display.show_welcome_screen().unwrap();

    loop{
        let qr_str = get_qr_str(&SECRET_HEX, USERID).unwrap();

        display.draw_qr_by_str(&qr_str).unwrap();
        log::info!("QR Code String: {}", qr_str);


        FreeRtos::delay_ms(1000);

    }

    // qr_generator::run_qr_generator(&SECRET_HEX, USERID);
}
