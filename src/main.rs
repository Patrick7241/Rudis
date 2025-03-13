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
use operations::hash::ops::{handle_hset_command};
use crate::operations::hash::ops::{handle_hdel_command, handle_hget_command, handle_hgetall_command};

#[derive(Clone)]
struct Storage{
    string_storage:Arc<Mutex<HashMap<String, String>>>,
    hash_storage:Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
}


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

    //  string类型存储
    let hash_table_string: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    // hash类型存储
    let hash_table_hash: Arc<Mutex<HashMap<String, HashMap<String, String>>>> = Arc::new(Mutex::new(HashMap::new()));

    let storage = Storage{
        string_storage:hash_table_string,
        hash_storage:hash_table_hash,
    };


    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        info!("接受 {} 地址的连接", addr);

        let storage = storage.clone();

        tokio::spawn(async move {
            handle_client(&mut socket, storage).await;
        });
    }
}

// 处理客户端连接的逻辑
async fn handle_client(socket: &mut tokio::net::TcpStream, storage: Storage) {
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
                    // string类型
                    Some(&"set") => handle_set_command(parts, socket, storage.string_storage.clone()).await,
                    Some(&"get") => handle_get_command(parts, socket,storage.string_storage.clone()).await,
                    Some(&"del")=> handle_del_command(parts, socket, storage.string_storage.clone()).await,

                    // hash类型
                    Some(&"hset")=> handle_hset_command(parts, socket, storage.hash_storage.clone()).await,
                    Some(&"hget")=> handle_hget_command(parts, socket, storage.hash_storage.clone()).await,
                    Some(&"hdel")=> handle_hdel_command(parts, socket, storage.hash_storage.clone()).await,
                    Some(&"hgetall")=>handle_hgetall_command(parts, socket, storage.hash_storage.clone()).await,
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


