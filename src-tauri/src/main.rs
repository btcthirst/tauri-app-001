// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(target_os = "windows"), windows_subsystem = "windows")]

fn main() {
    tauri_app_001_lib::run()
}
