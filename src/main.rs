#![feature(generic_const_exprs)]


mod display_sh1106;
mod qr_generator;
mod wifi;
mod bit_image;

use std::time::SystemTime;

use anyhow::Result;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};

use chrono::Local;


#[allow(unused_imports)]
use crate::qr_generator::{get_qr, get_qr_str}; // 
use crate::{qr_generator::calculate_totp, wifi::WifiManager};


use crate::bit_image::BitPixel;

static SECRET_HEX: [u8; 32] = qr_generator::hex_to_bytes(env!("SECRET_HEX")); // paySecret
static USERID: &str = env!("USER_ID"); // your żappka user id/ployId
static WIFI_SSID: &str = env!("WIFI_SSID");
static WIFI_PASS: &str = env!("WIFI_PASS");

const COMPILE_TIME: &str = env!("COMPILE_TIME");


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
    // let mut wifi_manager = WifiManager::new(peripherals.modem)?;
    // wifi_manager.connect(WIFI_SSID, WIFI_PASS)?;
    // wifi_manager.sync_time()?;

    let pins = peripherals.pins;
    let sda = pins.gpio8;
    let scl = pins.gpio7;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_dev = I2cDriver::new(i2c, sda, scl, &config)?;

    // Initialize display
    let mut display = display_sh1106::Display::new(i2c_dev).unwrap();
    display.print_metadata("Zabka esp".to_string(), COMPILE_TIME.to_string()).unwrap();

    FreeRtos::delay_ms(1000);

    loop {
        // let qr_str: String = get_qr_str(&SECRET_HEX, USERID).unwrap();

        // display.draw_qr_by_str(&qr_str).unwrap();
        // log::info!("QR Code String: {}", qr_str);

        let now = Local::now();
        let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let totp = calculate_totp(&SECRET_HEX).to_string();


        display.print_metadata( totp,time_str).unwrap();

        FreeRtos::delay_ms(2000);


        let qr = get_qr(&SECRET_HEX, USERID).unwrap();

        let binary_image = qr
            .clone()
            .render::<BitPixel>()
            .quiet_zone(false)
            .module_dimensions(3, 3)
            .build();

        let mut buf: [u8; 2048] = [0;2048];


        let buf_width = binary_image.set_bytearray(&mut buf);

        display.draw_from_buf(&buf, buf_width).unwrap();
        FreeRtos::delay_ms(10000);

        
        // display.draw_from_buf(***&&&binary_image.as_bytearray());
        // // log::info!("QR Code String: {}", qr_str);
    }
}
