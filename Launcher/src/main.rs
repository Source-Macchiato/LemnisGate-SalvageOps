#![windows_subsystem = "windows"]
use std::process::Command;

fn main() {
    Command::new("LemnisGate/Binaries/Win64/LemnisGate-Win64-Shipping.exe")
        .spawn()
        .expect("failed to execute process");
}