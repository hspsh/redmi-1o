mod display;
mod qr_generator;
mod wifi;

use anyhow::Result;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};

use crate::qr_generator::get_qr_str;
use crate::wifi::WifiManager;

static SECRET_HEX: [u8; 32] = qr_generator::hex_to_bytes(env!("SECRET_HEX")); // paySecret
static USERID: &str = env!("USER_ID"); // your żappka user id/ployId
static WIFI_SSID: &str = env!("WIFI_SSID");
static WIFI_PASS: &str = env!("WIFI_PASS");

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches
    // to the runtime implemented by esp-idf-sys might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Trace);

    log::info!("Starting application");
    let peripherals = Peripherals::take().unwrap();

    // Initialize WiFi and connect
    let mut wifi_manager = WifiManager::new(peripherals.modem)?;
    wifi_manager.connect(WIFI_SSID, WIFI_PASS)?;
    wifi_manager.sync_time()?;

    let pins = peripherals.pins;
    let sda = pins.gpio5;
    let scl = pins.gpio6;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_dev = I2cDriver::new(i2c, sda, scl, &config)?;

    // Initialize display
    let mut display = display::Display::new(i2c_dev).unwrap();
    display.show_welcome_screen().unwrap();

    loop {
        let qr_str = get_qr_str(&SECRET_HEX, USERID).unwrap();

        display.draw_qr_by_str(&qr_str).unwrap();
        log::info!("QR Code String: {}", qr_str);

        FreeRtos::delay_ms(1000);
    }
}
