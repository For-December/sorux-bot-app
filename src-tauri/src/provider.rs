use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

pub fn run_provider() {
    let _handler = thread::spawn(move || {
        let mut child = Command::new("bot-resources/provider/SoruxBot.Provider.QQ.exe")
            .current_dir("./bot-resources/provider")
            .stdout(Stdio::piped()) // 将标准输出重定向到管道
            .spawn() // 启动子进程
            .expect("Failed to start C# program");

        // 获取子进程的标准输出的句柄
        if let Some(output) = child.stdout.take() {
            let reader = BufReader::new(output);

            // 读取管道中的数据
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("Received: {}", line),
                    Err(e) => println!("Error reading line: {}", e),
                }
            }
        }

        // child.
        // 等待子进程结束
        let _ = child.wait().expect("Failed to wait on child");
    });
}
