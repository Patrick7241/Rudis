use std::collections::HashMap;
use std::sync::Arc;
use log::error;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// 检查命令格式
fn check_command_format(parts: &[&str], expected_len: usize) -> bool {
    parts.len() == expected_len
}

// 获取内部哈希表
async fn get_inner_map(
    parts: &[&str],
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
) -> Option<HashMap<String, String>> {
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

// 处理 HSET 命令
pub async fn handle_hset_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
) {
    if parts.len() <= 3 || parts.len() % 2 != 0 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();

    // 获取可变的哈希表锁
    let mut map = hash_table.lock().await;

    // 确保主哈希表中存在该键对应的内部哈希表
    let inner_map = map.entry(key.clone()).or_insert(HashMap::new());

    // 从第 2 个元素开始，每两个元素一组作为 field 和 value
    for i in (2..parts.len()).step_by(2) {
        let field = parts[i].trim_end_matches('\0').to_string();
        let value = parts[i + 1].trim_end_matches('\0').to_string();
        inner_map.insert(field, value);
    }

    socket.write_all("ok".as_bytes()).await.unwrap();
}

// 处理 HGET 命令
pub async fn handle_hget_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
) {
    if!check_command_format(&parts, 3) {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let field = parts[2].trim_end_matches('\0').to_string();
    if let Some(inner_map) = get_inner_map(&parts, socket, hash_table.clone()).await {
        if let Some(value) = inner_map.get(&field) {
            socket.write_all(value.as_bytes()).await.unwrap();
        } else {
            error!("未找到field");
            socket.write_all("未找到field".as_bytes()).await.unwrap();
        }
    }
}

// 处理 HDEL 命令
pub async fn handle_hdel_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
) {
    if!check_command_format(&parts, 3) {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let field = parts[2].trim_end_matches('\0').to_string();
    let mut lock_hash = hash_table.lock().await;
    if let Some(inner_map) = lock_hash.get_mut(&key) {
        if let Some(value) = inner_map.remove(&field) {
            socket.write_all(value.as_bytes()).await.unwrap();
        } else {
            error!("未找到field");
            socket.write_all("未找到field".as_bytes()).await.unwrap();
        }
    } else {
        error!("未找到key");
        socket.write_all("未找到key".as_bytes()).await.unwrap();
    }
}

// 处理 HGETALL 命令
pub async fn handle_hgetall_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
) {
    if!check_command_format(&parts, 2) {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    if let Some(inner_map) = get_inner_map(&parts, socket, hash_table.clone()).await {
        let mut res = String::new();
        for (field, value) in inner_map.iter() {
            let message = format!("{}:{}\n", field, value);
            res.push_str(&message);
        }
        socket.write_all(res.as_bytes()).await.unwrap();
    }
}
