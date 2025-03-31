use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => return,
        Ok(n) => {
            println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
            if let Err(e) = stream.write_all(&buffer[..n]).await {
                eprintln!("Failed to send response: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to read from client: {}", e),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_client(stream));
    }
}
