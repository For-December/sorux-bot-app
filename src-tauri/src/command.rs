use std::{
    fs::{self, File},
    io::Read,
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use tauri::Window;
use walkdir::WalkDir;

use crate::global_constants::{PROVIDER_DIR_PATH, WRAPPER_DIR_PATH};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn upload_file(file: Vec<u8>, filename: &str) -> Result<String, String> {
    use std::fs::File;
    use std::io::Write;

    let path = match filename.ends_with(".dll") {
        true => format!("{}/bin/{}", WRAPPER_DIR_PATH, filename),
        false => format!("{}/config/{}", WRAPPER_DIR_PATH, filename),
    };

    let mut output = File::create(path).map_err(|e| e.to_string())?;
    output.write_all(&file).map_err(|e| e.to_string())?;

    Ok("插件添加成功！".to_string())
}

#[tauri::command]
pub fn upload_plugin(
    json_file: Vec<u8>,
    dll_file: Vec<u8>,
    json_filename: String,
) -> Result<String, String> {
    // 同名上传
    upload_file(json_file, &json_filename).and_then(|_| {
        return upload_file(dll_file, &json_filename.replace(".json", ".dll"));
    })
}

#[derive(Serialize, Deserialize)]
pub struct PluginItem {
    name: String,
    privilege: Number,
    filename: Option<String>, // 记录文件原始路径
}

#[tauri::command]
pub fn get_plugins() -> Result<Vec<PluginItem>, String> {
    let mut res: Vec<PluginItem> = Vec::new(); // 创建一个存放名字的向量
    let path = format!("{}/config/", WRAPPER_DIR_PATH); // 指定要遍历的目录

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
pub fn del_plugins(filename: String) -> String {
    if let Err(e) = fs::remove_file(format!("{}/config/{}", WRAPPER_DIR_PATH, filename)) {
        return String::from(e.to_string());
    }

    if let Err(e) = fs::remove_file(format!(
        "{}/bin/{}",
        WRAPPER_DIR_PATH,
        filename.replace(".json", ".dll")
    )) {
        String::from(e.to_string())
    } else {
        String::new()
    }
}

#[tauri::command]
pub fn get_qrcode() -> Result<String, String> {
    let mut file = File::open(format!("{}/qr.png", PROVIDER_DIR_PATH))
        .map_err(|e| e.to_string())
        .unwrap();

    // 读取文件内容到一个 vector
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        Err(e.to_string())
    } else {
        // 将数据编码为 Base64 并返回
        Ok(STANDARD.encode(buffer))
    }
}

// 负载类型必须实现 `Serialize` 和 `Clone`。
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// 在 command 中初始化后台进程，并仅向使用该命令的窗口发出周期性事件
#[tauri::command]
pub fn init_process(window: Window) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        window
            .emit(
                "my-event",
                Payload {
                    message: "Tauri is awesome!".into(),
                },
            )
            .unwrap();
    });
}
