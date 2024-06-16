// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod command;
mod global_constants;
mod provider;

fn main() {
    provider::run_provider();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::upload_plugin,
            command::get_plugins,
            command::del_plugins,
            command::get_qrcode,
            command::init_process,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
