use hmac::{Hmac, Mac};
use qrcode::{EcLevel, QrCode};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

// use crate::bit_image::BitCanvas;

const JAVA_INT_MAX: u32 = 2_147_483_647;

pub const fn hex_to_bytes(hex: &str) -> [u8; 32] {
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

fn extract_int_from_bytes(arr: &[u8], index: usize) -> u32 {
    let bytes: [u8; 4] = arr[index..index + 4]
        .try_into()
        .expect("slice with incorrect length");
    u32::from_be_bytes(bytes)
}

pub fn calculate_totp(secret_hex: &[u8; 32]) -> u32 {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        / 30;

    let msg = ts.to_be_bytes();

    let mut mac = HmacSha1::new_from_slice(secret_hex)
        .expect("HMAC can take key of any size");
    mac.update(&msg);
    let output_bytes = mac.finalize().into_bytes();

    let offset = (output_bytes[output_bytes.len() - 1] & 0x0f) as usize;
    (extract_int_from_bytes(&output_bytes, offset) & JAVA_INT_MAX) % 1_000_000
}

fn make_qr(totp: u32, userid: &str) -> Result<QrCode, &'static str> {
    let url = format!(
        "https://srln.pl/view/dashboard?ploy={}&pay={:06}",
        userid, totp
    );

    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::L)
        .map_err(|_| "QR gen failed")?;

    Ok(code)
}

fn make_qr_str(totp: u32, userid: &str) -> Result<String, &'static str> {
    let qrcode = make_qr(totp, userid)?;

    let image = qrcode
        .render::<char>()
        .dark_color('.')
        .quiet_zone(false)
        .module_dimensions(2, 2)
        .build();

    Ok(image)
}

pub fn get_qr(
    secret_hex: &[u8; 32],
    userid: &str,
) -> Result<QrCode, &'static str> {
    let totp = calculate_totp(secret_hex);
    make_qr(totp, userid)
}

pub fn get_qr_str(
    secret_hex: &[u8; 32],
    userid: &str,
) -> Result<String, &'static str> {
    let totp = calculate_totp(secret_hex);
    make_qr_str(totp, userid)
}
