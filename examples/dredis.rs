use std::net::SocketAddr;

use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // build a listener
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("DRedis: Listening on {}", addr);

    loop {
        let (socket, remote_addr) = listener.accept().await?;
        info!("DRedis: Accepted connection from {}", remote_addr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_connection(socket, remote_addr).await {
                warn!(
                    "DRedis: Error processing connection: {}:{:?}",
                    remote_addr, e
                );
            }
        });
    }
}

async fn process_redis_connection(
    mut socket: tokio::net::TcpStream,
    remote_addr: SocketAddr,
) -> Result<()> {
    loop {
        socket.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match socket.try_read_buf(&mut buf) {
            Ok(0) => break, // EOF
            Ok(n) => {
                info!("DRedis: Read {} bytes", n);
                // Process the buffer here
                // For example, you can parse Redis commands and send responses
                let line = String::from_utf8_lossy(&buf);
                info!("DRedis: Received: {:?}", line);
                socket.write_all(b"+OK\r\n").await?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // WouldBlock means the socket is not ready for reading
                // You can handle this case if needed
                continue;
            }
            Err(e) => {
                info!("DRedis: Error reading from socket: {}", e);
                return Err(e.into());
            }
        }
    }

    warn!("DRedis: Connection closed by client: {}", remote_addr);
    Ok(())
}
