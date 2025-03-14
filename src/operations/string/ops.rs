use log::error;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// 检查命令格式
fn check_command_format(parts: &[&str], expected_len: usize) -> bool {
    parts.len() == expected_len
}

// 获取键对应的值
async fn get_value(
    parts: &[&str],
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, String>>>,
) -> Option<String> {
    let key = parts[1].trim_end_matches('\0').to_string();
    let lock_hash = hash_table.lock().await;
    match lock_hash.get(&key) {
        Some(v) => Some(v.clone()),
        None => {
            error!("未找到key");
            socket.write_all("未找到key".as_bytes()).await.unwrap();
            None
        }
    }
}

// 处理 SET 命令
pub async fn handle_set_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, String>>>,
) {
    if !check_command_format(&parts, 3) {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let value = parts[2].trim_end_matches('\0').to_string();

    hash_table.lock().await.insert(key, value);

    socket.write_all("ok".as_bytes()).await.unwrap();
}

// 处理 GET 命令
pub async fn handle_get_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, String>>>,
) {
    if !check_command_format(&parts, 2) {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }

    if let Some(value) = get_value(&parts, socket, hash_table.clone()).await {
        socket.write_all(value.as_bytes()).await.unwrap();
    }
}

// 处理 DEL 命令
pub async fn handle_del_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, String>>>,
) {
    if !check_command_format(&parts, 2) {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let mut lock_hash = hash_table.lock().await;
    if let Some(value) = lock_hash.remove(&key) {
        socket.write_all(value.as_bytes()).await.unwrap();
    } else {
        error!("未找到key");
        socket.write_all("未找到key".as_bytes()).await.unwrap();
    }
}
