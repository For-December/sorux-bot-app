use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::Arc,
    thread,
};

use crate::{global_channels::WRAPPER_LOGS_CHANNEL, global_constants::WRAPPER_DIR_PATH};

pub fn run_wrapper() -> Child {
    // 创建子进程启动 provider
    let mut child = Command::new(format!("{}/publish/SoruxBot.Wrapper.exe", WRAPPER_DIR_PATH))
        .current_dir(WRAPPER_DIR_PATH)
        .stdout(Stdio::piped()) // 将标准输出重定向到管道
        .spawn() // 启动子进程
        .expect("Failed to start C# wrapper program~");

    // 获取子进程的标准输出的句柄，并打印输出
    if let Some(output) = child.stdout.take() {
        let reader = BufReader::new(output);
        thread::spawn(move || {
            // 读取管道中的数据
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("Wrapper: {}", line);
                        {
                            let sender = Arc::clone(&WRAPPER_LOGS_CHANNEL.0);
                            let sender = sender.lock().unwrap();
                            sender.send(line.clone()).unwrap();
                            println!("sent..");
                        }
                    }
                    Err(e) => println!("Error reading line: {}", e),
                }
            }
        });
    }

    // child.
    // 等待子进程结束
    // let _ = child.wait().expect("Failed to wait on child")

    // 返回子进程句柄，后续操作
    child
}
