use std::{fs, process::Command};

fn main() {
    embuild::espidf::sysenv::output();

    let env_content = fs::read_to_string(".env").expect("Failed to read .env file");

    // Parse each line and set environment variables
    for line in env_content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            println!("cargo:rustc-env={}={}", key.trim(), value.trim());
        }
    }

    // Get current timestamp
    let output = Command::new("date")
        .args(["+%H:%M:%S"])
        .output()
        .expect("Failed to execute date command");

    let timestamp = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Set the compilation timestamp
    println!("cargo:rustc-env=COMPILE_TIME={}", timestamp);
}
