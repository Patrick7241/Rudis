use log::error;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// 处理 SADD 命令
pub async fn handle_sadd_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    storage: Arc<Mutex<HashMap<String, HashSet<String>>>>,
) {
    if parts.len() <= 2 {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let values: Vec<_> = parts[2..]
        .iter()
        .map(|s| s.trim_end_matches('\0').to_string())
        .collect();

    // 获取可变的哈希表锁
    let mut map = storage.lock().await;
    if let Some(set) = map.get_mut(&key) {
        for value in values {
            set.insert(value);
        }
    } else {
        let mut set = HashSet::new();
        for value in values {
            set.insert(value);
        }
        map.insert(key, set);
    }
    socket.write_all("ok".as_bytes()).await.unwrap();
}

// 处理 SISMEMBER 命令
pub async fn handle_sismember_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    storage: Arc<Mutex<HashMap<String, HashSet<String>>>>,
) {
    if parts.len() != 3 {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let value = parts[2].trim_end_matches('\0').to_string();
    let map = storage.lock().await;
    if let Some(set) = map.get(&key) {
        if set.contains(&value) {
            socket.write_all("1".as_bytes()).await.unwrap();
        } else {
            socket.write_all("0".as_bytes()).await.unwrap();
        }
    } else {
        socket.write_all("0".as_bytes()).await.unwrap();
    }
}

// 处理 SMEMBERS 命令
pub async fn handle_smembers_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    storage: Arc<Mutex<HashMap<String, HashSet<String>>>>,
) {
    if parts.len() != 2 {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let map = storage.lock().await;
    if let Some(set) = map.get(&key) {
        let mut values = set.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        values.sort();
        let result = values.join(" ");
        socket.write_all(result.as_bytes()).await.unwrap();
    } else {
        socket.write_all("".as_bytes()).await.unwrap();
    }
}

// 处理 SREM 命令
pub async fn handle_srem_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    storage: Arc<Mutex<HashMap<String, HashSet<String>>>>,
) {
    if parts.len() <= 2 {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let values: Vec<_> = parts[2..]
        .iter()
        .map(|s| s.trim_end_matches('\0').to_string())
        .collect();
    let mut map = storage.lock().await;
    let mut count = 0;
    if let Some(set) = map.get_mut(&key) {
        for value in values {
            if set.remove(&value) {
                count += 1;
            }
        }
        let res = format!("删除{}个元素", count);
        socket.write_all(res.as_bytes()).await.unwrap();
    } else {
        socket.write_all("未找到key".as_bytes()).await.unwrap();
    }
}
