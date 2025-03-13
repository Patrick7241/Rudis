use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 连接到服务器
    let mut stream = TcpStream::connect("127.0.0.1:6666").await?;

    // 发送消息到服务器
    let message = "Hello, server!";
    stream.write_all(message.as_bytes()).await?;

    // 接收服务器的响应
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    if n > 0 {
        let response = String::from_utf8_lossy(&buffer[0..n]);
        println!("Received from server: {}", response);
    }

    Ok(())
}