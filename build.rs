use std::process::Command;

fn main() {
    embuild::espidf::sysenv::output();

    // Get current timestamp
    let output = Command::new("date")
        .args(["+%H:%M:%S"])
        .output()
        .expect("Failed to execute date command");

    let timestamp = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Set the compilation timestamp
    println!("cargo:rustc-env=COMPILE_TIME={}", timestamp);
}
