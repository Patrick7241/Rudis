mod config;
mod log;
mod operations;

use std::collections::HashMap;
use std::sync::Arc;
use ::log::{error, info};
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use config::reader::reader;
use operations::string::ops::{handle_set_command,handle_del_command, handle_get_command};

#[tokio::main]
async fn main() {
    // 读取配置文件
    let config = match reader() {
        Some(c) => c,
        None => {
           error!("读取配置文件失败");
            std::process::exit(1);
        }
    };

    info!("读取配置文件成功");

    // 初始化日志库
    if let Err(e) =log::init::setup_logger(){
        error!("初始化日志库失败,{}",e);
        std::process::exit(1);
    }

    info!("初始化日志库成功");

    let listener = tokio::net::TcpListener::bind(config.rudis.address).await.unwrap();
    let hash_table: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        info!("接受 {} 地址的连接", addr);

        // 克隆 Arc 传入 tokio::spawn 线程
        let hash_table = Arc::clone(&hash_table);

        tokio::spawn(async move {
            handle_client(&mut socket, hash_table).await;
        });
    }
}

// 处理客户端连接的逻辑
async fn handle_client(socket: &mut tokio::net::TcpStream, hash_table: Arc<Mutex<HashMap<String, String>>>) {
    loop {
        let mut buffer = [0; 1024];
        match socket.read(&mut buffer).await {
            Ok(0) => {
                info!("客户端关闭");
                return;
            },
            Ok(_) => {
                let command = String::from_utf8_lossy(&buffer[..]);
                let command = command.to_lowercase();

                // 分割命令字符串
                let parts: Vec<&str> = command.split_whitespace().collect();


                match parts.get(0) {
                    Some(&"set") => handle_set_command(parts, socket, hash_table.clone()).await,
                    Some(&"get") => handle_get_command(parts, socket, hash_table.clone()).await,
                    Some(&"del")=> handle_del_command(parts, socket, hash_table.clone()).await,
                    _ => {
                        error!("未定义的指令类型");
                        let response = "未定义的指令类型";
                        socket.write_all(response.as_bytes()).await.unwrap();
                    }
                }
            },
            Err(e) => {
               error!("从服务端读取消息失败,{}",e);
                return;
            }
        }
    }
}


