use std::collections::HashMap;
use std::sync::Arc;
use log::error;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// 处理 SET 命令
pub async fn handle_set_command(parts: Vec<&str>, socket: &mut tokio::net::TcpStream, hash_table: Arc<Mutex<HashMap<String, String>>>) {
    if parts.len() != 3 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let value = parts[2].trim_end_matches('\0').to_string();

    hash_table.lock().await.insert(key, value);

    socket.write_all("ok".as_bytes()).await.unwrap();
}
// 处理 GET 命令
pub async fn handle_get_command(parts: Vec<&str>, socket: &mut tokio::net::TcpStream, hash_table: Arc<Mutex<HashMap<String, String>>>) {
    if parts.len() != 2 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0');

    let lock_hash = hash_table.lock().await;
    let value = match lock_hash.get(key) {
        Some(v) => v,
        None => {
            error!("未找到key");
            socket.write_all("未找到key".as_bytes()).await.unwrap();
            return;
        }
    };

    socket.write_all(value.as_bytes()).await.unwrap();
}

// 处理 DEL 命令
pub async fn handle_del_command(parts: Vec<&str>, socket: &mut tokio::net::TcpStream, hash_table: Arc<Mutex<HashMap<String, String>>>) {
    if parts.len() != 2 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0');

    let mut  lock_hash = hash_table.lock().await;
    let value = match lock_hash.remove(key) {
        Some(v) => v,
        None => {
            error!("未找到key");
            socket.write_all("未找到key".as_bytes()).await.unwrap();
            return;
        }
    };

    socket.write_all(value.as_bytes()).await.unwrap();
}
