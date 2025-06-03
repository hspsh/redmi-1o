mod qr_generator;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let secret_hex = ""; // paySecret
    let userid = ""; // your żappka user id/ployId
    qr_generator::run_qr_generator(secret_hex, userid);
}
