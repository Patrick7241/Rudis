mod config;
mod log;
mod operations;

use ::log::{error, info};
use config::reader::reader;
use operations::hash::ops::{
    handle_hdel_command, handle_hget_command, handle_hgetall_command, handle_hset_command,
};
use operations::list::ops::{
    handle_lpop_command, handle_lpush_command, handle_lrange_command, handle_rpop_command,
    handle_rpush_command,
};
use operations::set::ops::{
    handle_sadd_command,handle_sismember_command,handle_smembers_command,
    handle_srem_command,
};
use operations::string::ops::{handle_del_command, handle_get_command, handle_set_command};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

#[derive(Clone)]
struct Storage {
    string_storage: Arc<Mutex<HashMap<String, String>>>,
    hash_storage: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
    list_storage: Arc<Mutex<HashMap<String, VecDeque<String>>>>,
    set_storage: Arc<Mutex<HashMap<String, HashSet<String>>>>,
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
    if let Err(e) = log::init::setup_logger() {
        error!("初始化日志库失败,{}", e);
        std::process::exit(1);
    }

    info!("初始化日志库成功");

    let listener = tokio::net::TcpListener::bind(config.rudis.address)
        .await
        .unwrap();

    //  string类型存储
    let hash_table_string: Arc<Mutex<HashMap<String, String>>> =
        Arc::new(Mutex::new(HashMap::new()));
    // hash类型存储
    let hash_table_hash: Arc<Mutex<HashMap<String, HashMap<String, String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    // list类型存储
    let hash_table_list: Arc<Mutex<HashMap<String, VecDeque<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    // set类型存储
    let hash_table_set: Arc<Mutex<HashMap<String, HashSet<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let storage = Storage {
        string_storage: hash_table_string,
        hash_storage: hash_table_hash,
        list_storage: hash_table_list,
        set_storage: hash_table_set,
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
            }
            Ok(_) => {
                let command = String::from_utf8_lossy(&buffer[..]);
                let command = command.to_lowercase();

                // 分割命令字符串
                let parts: Vec<&str> = command.split_whitespace().collect();

                match parts.get(0) {
                    // string类型
                    Some(&"set") => {
                        handle_set_command(parts, socket, storage.string_storage.clone()).await
                    }
                    Some(&"get") => {
                        handle_get_command(parts, socket, storage.string_storage.clone()).await
                    }
                    Some(&"del") => {
                        handle_del_command(parts, socket, storage.string_storage.clone()).await
                    }

                    // hash类型
                    Some(&"hset") => {
                        handle_hset_command(parts, socket, storage.hash_storage.clone()).await
                    }
                    Some(&"hget") => {
                        handle_hget_command(parts, socket, storage.hash_storage.clone()).await
                    }
                    Some(&"hdel") => {
                        handle_hdel_command(parts, socket, storage.hash_storage.clone()).await
                    }
                    Some(&"hgetall") => {
                        handle_hgetall_command(parts, socket, storage.hash_storage.clone()).await
                    }

                    // list类型
                    Some(&"lpush") => {
                        handle_lpush_command(parts, socket, storage.list_storage.clone()).await
                    }
                    Some(&"rpush") => {
                        handle_rpush_command(parts, socket, storage.list_storage.clone()).await
                    }
                    Some(&"lpop") => {
                        handle_lpop_command(parts, socket, storage.list_storage.clone()).await
                    }
                    Some(&"rpop") => {
                        handle_rpop_command(parts, socket, storage.list_storage.clone()).await
                    }
                    Some(&"lrange") => {
                        handle_lrange_command(parts, socket, storage.list_storage.clone()).await
                    }

                    // set类型
                    Some(&"sadd") => {
                        handle_sadd_command(parts, socket, storage.set_storage.clone()).await
                    }
                    Some(&"sismember") => {
                        handle_sismember_command(parts, socket, storage.set_storage.clone()).await
                    }
                    Some(&"smembers") => {
                        handle_smembers_command(parts, socket, storage.set_storage.clone()).await
                    }
                    Some(&"srem") => {
                        handle_srem_command(parts, socket, storage.set_storage.clone()).await
                    }
                    _ => {
                        error!("未定义的指令类型");
                        let response = "未定义的指令类型";
                        socket.write_all(response.as_bytes()).await.unwrap();
                    }
                }
            }
            Err(e) => {
                error!("从服务端读取消息失败,{}", e);
                return;
            }
        }
    }
}
