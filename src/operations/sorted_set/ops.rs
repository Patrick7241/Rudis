use log::error;
use skiplist::OrderedSkipList;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

// 处理 ZADD 命令
pub async fn handle_zadd_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, OrderedSkipList<(f64, String)>>>>,
    hash: Arc<Mutex<HashMap<String, HashMap<String, f64>>>>,
) {
    if parts.len() <= 3 || parts.len() % 2 != 0 {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let mut score_member_pairs = Vec::new();
    for i in (2..parts.len()).step_by(2) {
        let score = parts[i].trim_end_matches('\0').parse::<f64>().unwrap();
        let member = parts[i + 1].trim_end_matches('\0').to_string();
        score_member_pairs.push((score, member));
    }
    let mut map = hash_table.lock().await;
    let mut hash_map = hash.lock().await;

    let skip_key = key.clone();
    let smp = score_member_pairs.clone();
    // 存储数据到跳表中
    if let Some(set) = map.get_mut(&skip_key) {
        for (score, member) in smp {
            set.insert((score, member.clone()));
        }
    } else {
        let mut set = OrderedSkipList::new();
        for (score, member) in smp {
            set.insert((score, member));
        }
        map.insert(skip_key, set);
    }
    let hash_key = key.clone();
    let hash_smp = score_member_pairs.clone();
    // 存储数据到hash中
    let inner_map = hash_map.entry(hash_key).or_insert(HashMap::new());
    for (score, member) in hash_smp {
        inner_map.insert(member, score);
    }

    socket.write_all("ok".as_bytes()).await.unwrap();
}

// 处理 ZRANGE 命令
pub async fn handle_zrange_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, OrderedSkipList<(f64, String)>>>>,
) {
    if parts.len() != 4 {
        error!("命令格式不符合！");
        socket
            .write_all("命令格式不符合！".as_bytes())
            .await
            .unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let start = parts[2].trim_end_matches('\0').parse::<i32>().unwrap();
    let end = parts[3].trim_end_matches('\0').parse::<i32>().unwrap();
    let mut map = hash_table.lock().await;

    if start == 0 && end == -1 {
        let mut res = String::new();
        if let Some(set) = map.get_mut(&key) {
            for value in set.iter() {
                let message = format!("元素是： {}， 分数是： {}\n", value.1, value.0);
                res.push_str(&message);
            }
        }
        socket.write_all(res.as_bytes()).await.unwrap();
        return;
    }

    if let Some(set) = map.get_mut(&key) {
        let mut res = String::new();
        for i in start..=end {
            if let Some(value) = set.get(i as usize) {
                let message = format!("元素是： {}， 分数是： {}\n", value.1, value.0);
                res.push_str(&message);
            }
        }
        socket.write_all(res.as_bytes()).await.unwrap();
    } else {
        error!("未找到key");
    }
}

// 处理 ZREM 命令
pub async fn handle_zrem_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, OrderedSkipList<(f64, String)>>>>,
    hash: Arc<Mutex<HashMap<String, HashMap<String, f64>>>>,
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
    let member = parts[2].trim_end_matches('\0').to_string();

    let mut map = hash_table.lock().await;
    let mut hash_map = hash.lock().await;

    if let Some(inner_map) = hash_map.get_mut(&key) {
        if let Some(score) = inner_map.remove(&member) {
            if let Some(set) = map.get_mut(&key) {
                set.remove(&(score, member.clone()));
            }
            socket.write_all("ok".as_bytes()).await.unwrap();
            return;
        } else {
            error!("未找到member");
            socket.write_all("未找到member".as_bytes()).await.unwrap();
        }
    } else {
        error!("未找到key");
        socket
            .write_all("未找到key或member".as_bytes())
            .await
            .unwrap();
    }
}

// 处理 ZSCORE 命令
pub async fn handle_zscore_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String, HashMap<String, f64>>>>,
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
    let member = parts[2].trim_end_matches('\0').to_string();
    let mut map = hash_table.lock().await;
    if let Some(inner_map) = map.get_mut(&key) {
        if let Some(score) = inner_map.get(&member) {
            let message = format!("{}", score);
            socket.write_all(message.as_bytes()).await.unwrap();
            return;
        } else {
            error!("未找到member");
            socket.write_all("未找到member".as_bytes()).await.unwrap();
        }
    } else {
        error!("未找到key");
        socket.write_all("未找到key".as_bytes()).await.unwrap();
    }
}
