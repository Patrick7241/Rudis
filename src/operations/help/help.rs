use tokio::io::AsyncWriteExt;

pub async fn help(socket: &mut tokio::net::TcpStream) {
    let res = "支持命令：SET、GET、HSET、HGET、LPUSH等各类数据操作";
    socket.write_all(res.as_bytes()).await.unwrap();
}