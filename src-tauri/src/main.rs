// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::{to_string, Number};
use walkdir::WalkDir;

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

#[derive(Serialize, Deserialize)]
struct PluginItem {
    name: String,
    privilege: Number,
    filename: Option<String>, // 记录文件原始路径
}

#[tauri::command]
fn get_plugins() -> Result<Vec<PluginItem>, String> {
    let mut res: Vec<PluginItem> = Vec::new(); // 创建一个存放名字的向量
    let path = "./bot-resources/wrapper/config/"; // 指定要遍历的目录

    // 遍历目录和子目录中的所有文件
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        // 检查文件是否是 JSON 文件
        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("json") {
            // 读取文件内容
            let data = fs::read_to_string(path).map_err(|e| e.to_string()).unwrap();
            println!("{}", data);
            // 解析 JSON 数据
            let mut item: PluginItem = serde_json::from_str(&data.to_lowercase())
                .map_err(|e| e.to_string())
                .unwrap();

            println!(
                "{}",
                path.file_name().and_then(std::ffi::OsStr::to_str).unwrap()
            );
            item.filename = Some(
                path.file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap()
                    .to_string(),
            );
            res.push(item);
        }
    }

    Ok(res)
}

#[tauri::command]
fn del_plugins(filename: String) -> String {
    if let Err(e) = fs::remove_file(format!("./bot-resources/wrapper/config/{}", filename)) {
        return String::from(e.to_string());
    }

    if let Err(e) = fs::remove_file(format!("./bot-resources/wrapper/bin/{}", filename)) {
        String::from(e.to_string())
    } else {
        String::new()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            upload_file,
            get_plugins,
            del_plugins
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
