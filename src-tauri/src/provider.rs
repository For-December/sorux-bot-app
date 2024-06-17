use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::thread;

use crate::global_channels::WRAPPER_BOT_LOGIN_CHANNEL;
use crate::global_constants::PROVIDER_DIR_PATH;

pub fn run_provider() -> Child {
    // 创建子进程启动 provider
    let mut child = Command::new(format!(
        "{}/publish/SoruxBot.Provider.QQ.exe",
        PROVIDER_DIR_PATH
    ))
    .current_dir(PROVIDER_DIR_PATH)
    .stdout(Stdio::piped()) // 将标准输出重定向到管道
    .spawn() // 启动子进程
    .expect("Failed to start C# provider program");

    // 获取子进程的标准输出的句柄，并打印输出
    if let Some(output) = child.stdout.take() {
        let reader = BufReader::new(output);
        thread::spawn(move || {
            // 读取管道中的数据
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("Provider: {}", line);
                        if line.contains("Account has logged in") {
                            println!("登陆上了，发信号");
                            {
                                let sender = Arc::clone(&WRAPPER_BOT_LOGIN_CHANNEL.0);
                                let sender = sender.lock().unwrap();
                                sender.send(true).unwrap();
                            }
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
