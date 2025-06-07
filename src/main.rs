#![feature(generic_const_exprs)]

mod bit_image;
mod buzzer;
mod display_sh1106;
mod qr_generator;
mod wifi;

use std::time::SystemTime;

use anyhow::Result;
use esp_idf_hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    prelude::*,
};

use chrono::{Datelike, Local};

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
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Trace);

    log::info!("Starting application");
    let peripherals = Peripherals::take().unwrap();

    let pins = peripherals.pins;
    let sda = pins.gpio8;
    let scl = pins.gpio7;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c_dev = I2cDriver::new(i2c, sda, scl, &config)?;

    // Initialize buzzer and play a single tone
    let buzzer = buzzer::Buzzer::new(pins.gpio2, peripherals.ledc)?;
    buzzer.enqueue_tones(&[buzzer::BuzzerTone {
        freq_hz: 200,
        duration_ms: 3000,
    }]);

    // Initialize display
    let mut display = display_sh1106::Display::new(i2c_dev).unwrap();
    display
        .print_metadata("connecting to WiFi".to_string(), WIFI_SSID.to_string())
        .unwrap();

    let mut wifi_manager = WifiManager::new(peripherals.modem)?;
    wifi_manager.connect(WIFI_SSID, WIFI_PASS)?;
    wifi_manager.init_sync()?;

    wait_sntp_sync(&mut display, 30);
    
    FreeRtos::delay_ms(1000);

    loop {
        display_totp_meta(&mut display);
        FreeRtos::delay_ms(2000);

        display_qr(&mut display);
        FreeRtos::delay_ms(10000);

    }
}

fn wait_sntp_sync(display: &mut display_sh1106::Display, _timeout_sec: u64) {
    loop {
        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Current time: {}", formatted_time);

        display.print_metadata(
            "wait timesync".to_string(),
            formatted_time,
        ).unwrap();

        if now.year() > 2000 {
            return;
        }

        FreeRtos::delay_ms(1000);
    }
}

fn display_qr(display: &mut display_sh1106::Display) {
    let qr = get_qr(&SECRET_HEX, USERID).unwrap();

    let binary_image = qr
        .clone()
        .render::<BitPixel>()
        .quiet_zone(false)
        .module_dimensions(3, 3)
        .build();

    let mut buf: [u8; 2048] = [0; 2048];

    let buf_width = binary_image.set_bytearray(&mut buf);

    display.draw_from_buf(&buf, buf_width).unwrap();
}

fn display_totp_meta(display: &mut display_sh1106::Display) {
    let now = Local::now();
    let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let totp = calculate_totp(&SECRET_HEX);

    let totp_str = format!("{:06}", totp);

    display.print_metadata(totp_str, time_str).unwrap();
}
