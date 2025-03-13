mod config;

use std::fmt::{Debug};
use tokio::io::AsyncWriteExt;
use config::reader::reader;

#[tokio::main]
async fn main() {
    // 读取配置文件
    let config = match reader() {
        Some(c) => c,
        None => {
            println!("Failed to read config.yaml");
            std::process::exit(1);
        }
    };


    let listener = tokio::net::TcpListener::bind(config.rudis.address).await.unwrap();


    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from: {}", addr);

        // 创建一个任务来处理每个客户端连接
        tokio::spawn(async move {
            // 向客户端发送一个简单的响应
            let response = "Hello from server!";
            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Failed to send response to {}: {}", addr, e);
            } else {
                println!("Sent response to {}", addr);
            }
        });
    }
}

