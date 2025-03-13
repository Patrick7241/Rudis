use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use log::error;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// 处理 LPUSH 命令，从左侧添加元素
pub async fn handle_lpush_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,VecDeque<String>>>>,
) {
    if parts.len() < 3 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let mut lock_hash = hash_table.lock().await;
    if let Some(inner_list) = lock_hash.get_mut(&key) {
        // 如果存在直接追加
        for i in 2..parts.len() {
            inner_list.push_front(parts[i].trim_end_matches('\0').to_string());
        }
    } else {
        // 如果不存在创建一个新的
        lock_hash.insert(key.clone(),VecDeque::new());
        for i in 2..parts.len() {
            lock_hash.get_mut(&key).unwrap().push_front(parts[i].trim_end_matches('\0').to_string());
        }
    }
    socket.write_all("ok".as_bytes()).await.unwrap();
}

// 处理 RPUSH 命令，从右侧添加元素
pub async fn handle_rpush_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,VecDeque<String>>>>,
) {
    if parts.len() < 3 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let mut lock_hash = hash_table.lock().await;
    if let Some(inner_list) = lock_hash.get_mut(&key) {
        // 如果存在直接追加
        for i in 2..parts.len() {
            inner_list.push_back(parts[i].trim_end_matches('\0').to_string());
        }
    } else {
        // 如果不存在创建一个新的
        lock_hash.insert(key.clone(),VecDeque::new());
        for i in 2..parts.len() {
            lock_hash.get_mut(&key).unwrap().push_back(parts[i].trim_end_matches('\0').to_string());
        }
    }
    socket.write_all("ok".as_bytes()).await.unwrap();
}

// 处理 LPOP 命令，从左侧添加元素
pub async fn handle_lpop_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,VecDeque<String>>>>,
) {
    if parts.len() !=2 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let mut lock_hash = hash_table.lock().await;
    if let Some(inner_list) = lock_hash.get_mut(&key) {
        if let Some(value) = inner_list.pop_front() {
            socket.write_all(value.as_bytes()).await.unwrap();
        } else {
            error!("元素不存在");
            socket.write_all("元素不存在".as_bytes()).await.unwrap();
        }
    } else {
        error!("未找到key");
    }
}

// 处理 RPOP 命令，从左侧添加元素
pub async fn handle_rpop_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,VecDeque<String>>>>,
) {
    if parts.len() !=2 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let mut lock_hash = hash_table.lock().await;
    if let Some(inner_list) = lock_hash.get_mut(&key) {
        if let Some(value) = inner_list.pop_back() {
            socket.write_all(value.as_bytes()).await.unwrap();
        } else {
            error!("元素不存在");
            socket.write_all("元素不存在".as_bytes()).await.unwrap();
        }
    } else {
        error!("未找到key");
    }
}

// 处理 LRANGE 命令，获取列表范围内的所有元素  0,-1返回所有元素
pub async fn handle_lrange_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,VecDeque<String>>>>,
) {
    if parts.len() !=4 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();

    if let Some(inner_list) = hash_table.lock().await.get(&key) {
        let start = parts[2].trim_end_matches('\0').parse::<i32>().unwrap();
        let end = parts[3].trim_end_matches('\0').parse::<i32>().unwrap();

        // 处理返回所有的特殊情况
        if start == 0 && end == -1 {
            let mut res = String::new();
            for value in inner_list.iter() {
                let message = format!("{}\n", value);
                res.push_str(&message);
            }
            socket.write_all(res.as_bytes()).await.unwrap();
            return;
        }

        let mut res = String::new();
        for i in start..=end {
            if let Some(value) = inner_list.get(i as usize) {
               let message = format!("{}\n", value);
                res.push_str(&message);
            }
        }
        socket.write_all(res.as_bytes()).await.unwrap();
    }else{
        error!("未找到key");
    }
}

