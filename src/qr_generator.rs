use embedded_graphics::image::{Image, ImageDrawable, ImageRaw, ImageRawLE};
use embedded_graphics::pixelcolor::BinaryColor;
use hmac::{Hmac, Mac};
use qrcode::render::{image, unicode};
use qrcode::{EcLevel, QrCode};
use sha1::Sha1;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// use image::color::Luma;


type HmacSha1 = Hmac<Sha1>;

const JAVA_INT_MAX: u32 = 2_147_483_647;

fn extract_int_from_bytes(arr: &[u8], index: usize) -> u32 {
    let bytes: [u8; 4] = arr[index..index + 4]
        .try_into()
        .expect("slice with incorrect length");
    u32::from_be_bytes(bytes)
}

fn calculate_totp(secret_hex: &[u8;32]) -> u32 {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        / 30;

    let msg = ts.to_be_bytes();

    let mut mac = HmacSha1::new_from_slice(secret_hex).expect("HMAC can take key of any size");
    mac.update(&msg);
    let output_bytes = mac.finalize().into_bytes();

    let offset = (output_bytes[output_bytes.len() - 1] & 0x0f) as usize;
    (extract_int_from_bytes(&output_bytes, offset) & JAVA_INT_MAX) % 1_000_000
}

fn make_qr_str(totp: u32, userid: &str) -> Result<String, &'static str> {
    let url = format!(
        "https://srln.pl/view/dashboard?ploy={}&pay={:06}",
        userid, totp
    );

    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::L)
        .map_err(|_| "QR gen failed")?;

    let image = code.render::<char>()
        .dark_color('.')
        .quiet_zone(false)
        .module_dimensions(1, 1)
        .build();

    Ok(image)
}

// fn make_qr_buffer(totp: u32, userid: &str) -> [u8; 1024] {
//     let dots = make_qr(totp, userid).unwrap_or_else(|_| "QR gen failed".to_string());

//     let mut buffer = [0u8; 1024];
    
// }

pub fn get_qr_str(secret_hex: &[u8;32], userid: &str) -> Result<String, &'static str> {
    let totp = calculate_totp(secret_hex);
    make_qr_str(totp, userid)
}

pub fn run_qr_generator(secret_hex: &[u8;32], userid: &str) {
    loop {
        let totp = calculate_totp(secret_hex);
        println!("{}", totp);
        
        if let Ok(qr) = make_qr_str(totp, userid) {
            println!("{}", qr);
        }
        thread::sleep(Duration::from_secs(2));
    }
}
