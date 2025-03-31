// TCP listener
// Author: Jacob Hall
// Date: 03/30/2025
// Version 1.0
// Program handles multiple clients asynchronously and echos back
// whatever is said into the console. This related to VPNs because
// they also read and write network packs to encrypt the traffic.

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_client(mut stream: TcpStream) {
    // Holds 1024 bytes of data in 32 bit chunks for 32 unique areas
    let mut buffer = [0; 1024];

    // Reads data asynchronously
    match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => return, // client disconnected
        Ok(n) => {
            println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
            // Echos the data back to the console
            if let Err(e) = stream.write_all(&buffer[..n]).await {
                eprintln!("Failed to send response: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to read from client: {}", e),
    }
}

// Sets up the server
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Listen for TCP connections
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (stream, _) = listener.accept().await?; // wait for new client to connect
        tokio::spawn(handle_client(stream)); // allows multiple clients at once
    }
}
