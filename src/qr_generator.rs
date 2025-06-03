use hmac::{Hmac, Mac};
use qrcode::render::unicode;
use qrcode::{EcLevel, QrCode};
use sha1::Sha1;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

const JAVA_INT_MAX: u32 = 2_147_483_647;

fn extract_int_from_bytes(arr: &[u8], index: usize) -> u32 {
    let bytes: [u8; 4] = arr[index..index + 4]
        .try_into()
        .expect("slice with incorrect length");
    u32::from_be_bytes(bytes)
}

fn calculate_totp(_secret_hex: &str) -> u32 {
    // set to empty
    let secret: [u8; 32] = [0; 32];

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        / 30;

    let msg = ts.to_be_bytes();

    let mut mac = HmacSha1::new_from_slice(&secret).expect("HMAC can take key of any size");
    mac.update(&msg);
    let output_bytes = mac.finalize().into_bytes();

    let offset = (output_bytes[output_bytes.len() - 1] & 0x0f) as usize;
    (extract_int_from_bytes(&output_bytes, offset) & JAVA_INT_MAX) % 1_000_000
}

fn make_qr(totp: u32, userid: &str) -> Result<(), &'static str> {
    // todo: add loyal support and detection
    let url = format!(
        "https://srln.pl/view/dashboard?ploy={}&pay={:06}",
        userid, totp
    );

    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::Q)
        .map_err(|_| "QR gen failed")?;

    let string = code
        .render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .module_dimensions(1, 1)
        .build();

    println!("{}", string);

    Ok(())
}

pub fn run_qr_generator(secret_hex: &str, userid: &str) {
    loop {
        let _res = make_qr(calculate_totp(secret_hex), userid);
        println!("{}", calculate_totp(secret_hex));
        thread::sleep(Duration::from_secs(2));
    }
}
