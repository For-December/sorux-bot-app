// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{process::Child, sync::mpsc, thread};

use global_channels::CHILD_PROCESS_MAP;
use global_constants::{PROVIDER_CHILD_NAME, WRAPPER_CHILD_NAME};

mod command;
mod global_channels;
mod global_constants;
mod provider;
mod wrapper;

fn main() {
    // 用来关闭子进程的通道
    let (tx, rx) = mpsc::channel::<bool>();

    let provider_child = provider::run_provider();
    let wrapper_child = wrapper::run_wrapper();

    {
        let mut map = CHILD_PROCESS_MAP.lock().unwrap();
        map.insert(PROVIDER_CHILD_NAME.to_string(), provider_child);
        map.insert(WRAPPER_CHILD_NAME.to_string(), wrapper_child);
    }

    thread::spawn(|| {
        // 在主线程中作为消费者接收并处理布尔值
        for _ in rx {
            println!("接收到退出信号！");

            let mut provider_child: Child;
            let mut wrapper_child: Child;
            {
                // 每次拿到锁一定是操作两个值
                let mut map = CHILD_PROCESS_MAP.lock().unwrap();
                provider_child = map.remove(PROVIDER_CHILD_NAME).unwrap();
                wrapper_child = map.remove(WRAPPER_CHILD_NAME).unwrap();
            }

            // 结束子进程
            let _ = provider_child.kill().expect("Failed to kill child process");
            let _ = wrapper_child.kill().expect("Failed to kill child process");

            println!("provider and wrapper children process all exit!");

            // 等待子进程结束
            let _ = provider_child.wait().expect("Failed to wait on child");
            let _ = wrapper_child.kill().expect("Failed to kill child process");
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
            command::logout,
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
