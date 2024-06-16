// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn upload_file(file: Vec<u8>, filename: String) -> Result<String, String> {
    use std::fs::File;
    use std::io::Write;
    

    let path = match filename.ends_with(".dll") {
        true => format!("./bot-resources/wrapper/bin/{}", filename),
        false => format!("./bot-resources/wrapper/config/{}", filename),
    };

    let mut output = File::create(path).map_err(|e| e.to_string())?;
    output.write_all(&file).map_err(|e| e.to_string())?;

    Ok("文件上传成功".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, upload_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
