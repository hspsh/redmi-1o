mod qr_generator;
mod display;

use display::Display;

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

static SECRET_HEX: [u8; 32] = hex_to_bytes(env!("SECRET_HEX"));  // paySecret
static USERID: &str = env!("USER_ID"); // your żappka user id/ployId

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    // Initialize the display
    // let mut display = display::init_display().expect("Failed to initialize display");
    // display.clear().expect("Failed to clear display");
    // display.show_text("Zabka ESP", 0, 0).expect("Failed to show text");

    qr_generator::run_qr_generator(&SECRET_HEX, USERID);
}
