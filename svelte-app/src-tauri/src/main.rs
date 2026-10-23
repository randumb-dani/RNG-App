#![cfg_attr(
    all(debug_assertions, not(test), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    if let Err(e) = rng_app::run() {
        eprintln!("Error while running tauri application: {}", e);
        std::process::exit(1);
    }
}
