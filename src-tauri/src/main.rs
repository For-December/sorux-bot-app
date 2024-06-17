// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{sync::mpsc, thread};

mod command;
mod global_constants;
mod provider;

fn main() {
    // 用来关闭子进程的通道
    let (tx, rx) = mpsc::channel::<bool>();

    let mut provider_child = provider::run_provider();
    thread::spawn(move || {
        // 在主线程中作为消费者接收并处理布尔值
        for _ in rx {
            println!("接收到退出信号！");
            // 结束子进程
            let _ = provider_child.kill().expect("Failed to kill child process");

            // // 等待子进程结束
            let _ = provider_child.wait().expect("Failed to wait on child");
            break;
        }
        println!("监听线程退出！")
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::upload_plugin,
            command::get_plugins,
            command::del_plugins,
            command::watch_qrcode,
            command::init_process,
        ])
        .on_window_event(move |event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { .. } => {
                    // api.prevent_close();
                    println!("窗口关闭！");

                    tx.send(true).unwrap();

                    println!("程序结束！");
                    // let _ = event.window().close();
                }
                _ => {} // TODO
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
