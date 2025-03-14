use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接到服务器
    let mut stream = TcpStream::connect("127.0.0.1:6666").await?;
    println!("连接到127.0.0.1:6666");

    loop {
        // 从用户输入读取命令
        print!("127.0.0.1:6666: ");
        io::stdout().flush().unwrap(); // 确保立即显示提示
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        // 去掉换行符
        command = command.trim().to_string();

        // 如果用户输入 "exit"，则退出
        if command.to_lowercase() == "exit" {
            println!("正在退出...");
            break;
        }

        // 发送命令到服务器
        stream.write_all(command.as_bytes()).await?;

        // 接收服务器的响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        if n > 0 {
            let response = String::from_utf8_lossy(&buffer[..n]);
            println!("服务器的回复: {}", response);
        }
    }

    Ok(())
}
