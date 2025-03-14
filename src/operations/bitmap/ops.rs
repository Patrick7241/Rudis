use std::collections::HashMap;
use std::sync::Arc;
use bitvec::prelude::BitVec;
use log::error;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

const MAX_OFFSET: usize = 1024;

// 处理 SETBIT 命令
pub async fn handle_setbit_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,BitVec>>>,
){
    if parts.len() != 4 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }

    let key = parts[1].trim_end_matches('\0').to_string();
    let offset = parts[2].trim_end_matches('\0').parse::<usize>().unwrap();
    let value = parts[3].trim_end_matches('\0').parse::<u8>().unwrap();
    let mut lock_hash = hash_table.lock().await;
    if let Some(inner_list) = lock_hash.get_mut(&key) {
        if offset >= inner_list.len(){
            error!("偏移量超出范围");
            socket.write_all("偏移量超出范围".as_bytes()).await.unwrap();
            return;
        }
        if value==1{
            inner_list.set(offset,true);
        }else{
            inner_list.set(offset,false);
        }
        socket.write_all("ok".as_bytes()).await.unwrap();
    }else{
        let bit_vec=BitVec::repeat(false,MAX_OFFSET);
        lock_hash.insert(key.clone(),bit_vec);
        lock_hash.get_mut(&key).unwrap().set(offset,value==1);
        socket.write_all("ok".as_bytes()).await.unwrap();
    }
}

// 处理 GETBIT 命令
pub async fn handle_getbit_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,BitVec>>>,
){
    if parts.len() != 3 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let offset = parts[2].trim_end_matches('\0').parse::<usize>().unwrap();
    let lock_hash = hash_table.lock().await;
    if let Some(inner_list) = lock_hash.get(&key) {
        if offset >= inner_list.len(){
            error!("偏移量超出范围");
            socket.write_all("偏移量超出范围".as_bytes()).await.unwrap();
            return;
        }
        let res = match inner_list.get(offset) {
            Some(bit_ref) => {
                if *bit_ref {
                    "1"
                } else {
                    "0"
                }
            }
            None => {
                "0"
            }
        };
        socket.write_all(res.as_bytes()).await.unwrap();
   }else{
        socket.write_all("0".as_bytes()).await.unwrap();
    }
}

// 处理 BITCOUNT 命令
pub async fn handle_bitcount_command(
    parts: Vec<&str>,
    socket: &mut tokio::net::TcpStream,
    hash_table: Arc<Mutex<HashMap<String,BitVec>>>,
){
    if parts.len() != 2 {
        error!("命令格式不符合！");
        socket.write_all("命令格式不符合！".as_bytes()).await.unwrap();
        return;
    }
    let key = parts[1].trim_end_matches('\0').to_string();
    let lock_hash = hash_table.lock().await;
    if let Some(inner_list) = lock_hash.get(&key) {
        let count=inner_list.count_ones();
        let res=format!("位图中1的数量是：{}",count);
        socket.write_all(res.as_bytes()).await.unwrap();
    }else{
        socket.write_all("0".as_bytes()).await.unwrap();
    }
}