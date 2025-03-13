mod config;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use config::reader::reader;

#[tokio::main]
async fn main() {
    // 读取配置文件
    let config = match reader() {
        Some(c) => c,
        None => {
            println!("读取配置文件失败");
            std::process::exit(1);
        }
    };

    let listener = tokio::net::TcpListener::bind(config.rudis.address).await.unwrap();
    let hash_table: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Accepted connection from: {}", addr);

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
                println!("客户端关闭");
                return;
            },
            Ok(_) => {
                let command = String::from_utf8_lossy(&buffer[..]);
                println!("接受客户端的消息: {}", command);
                let command = command.to_lowercase();

                // 分割命令字符串
                let parts: Vec<&str> = command.split_whitespace().collect();

                match parts.get(0) {
                    Some(&"set") => handle_set_command(parts, socket, hash_table.clone()).await,
                    Some(&"get") => handle_get_command(parts, socket, hash_table.clone()).await,
                    _ => {
                        println!("未定义的指令类型");
                        let response = "未定义的指令类型";
                        socket.write_all(response.as_bytes()).await.unwrap();
                    }
                }
            },
            Err(e) => {
                eprintln!("从服务端读取消息失败: {}", e);
                return;
            }
        }
    }
}

// 处理 SET 命令
async fn handle_set_command(parts: Vec<&str>, socket: &mut tokio::net::TcpStream, hash_table: Arc<Mutex<HashMap<String, String>>>) {
    if parts.len() != 3 {
        println!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let value = parts[2].trim_end_matches('\0').to_string();

    println!("要存储的key是 : {}", key);
    println!("要存储的value是 : {}", value);

    hash_table.lock().await.insert(key, value);

    // 查看当前存储状态
    println!("当前存储状态: {:#?}", hash_table.lock().await);

    socket.write_all("ok".as_bytes()).await.unwrap();
}

// 处理 GET 命令
async fn handle_get_command(parts: Vec<&str>, socket: &mut tokio::net::TcpStream, hash_table: Arc<Mutex<HashMap<String, String>>>) {
    if parts.len() != 2 {
        println!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0');

    let lock_hash = hash_table.lock().await;
    let value = match lock_hash.get(key) {
        Some(v) => v,
        None => {
            println!("未找到key: {}", key);
            socket.write_all("未找到key".as_bytes()).await.unwrap();
            return;
        }
    };

    socket.write_all(value.as_bytes()).await.unwrap();
}
