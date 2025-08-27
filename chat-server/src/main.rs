use std::net::SocketAddr;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await?;

    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);

    loop {
        let (stream, addr) = tcp_listener.accept().await?;
        let tx = tx.clone();
        let rx: broadcast::Receiver<(String, SocketAddr)> = tx.subscribe();

        tokio::spawn(async move {
            handle_client(stream, addr, tx, rx).await;
        });
    }
}

async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<(String, SocketAddr)>,
    mut rx: broadcast::Receiver<(String, SocketAddr)>,
) {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let mut buffer = [0; 1024];

    println!("Client connected {addr:?}");

    loop {
        tokio::select! {
            result = reader.read(&mut buffer) => {
                match result {
                    Ok(0) => break, // Disconnected
                    Ok(bytes_read) => {
                        let msg = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

                        if !msg.is_empty() {
                            if let Err(e) = tx.send((msg.clone(), addr)) {
                                eprintln!("Failed to broadcast: {e}");
                            }

                            println!("Received {msg}");
                            let _ = writer.write_all(format!("You: {msg}\n").as_bytes()).await;
                        }
                    },
                    Err(_) => break, // Disconnected
                }
            },
            result = rx.recv() => {
                match result {
                    Ok((msg, sender_addr)) if sender_addr != addr => {
                        let _ = writer.write_all(format!("{sender_addr}: {msg}\n").as_bytes()).await;
                        let _ = writer.flush().await;
                    },
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // Handle lagging (optional: log or skip)
                        println!("Client {addr:?} lagged behind");
                    }
                    _ => {}
                }
            }
        }
    }
}
