use std::process::exit;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:8080").await?;

    // Hello World
    let message = "Hello, Server!";
    stream.write_all(message.as_bytes()).await?;

    let (mut read_stream, mut write_stream) = stream.into_split();

    // Lendo os dados recebidos
    tokio::spawn(async move {
        let mut reader = BufReader::new(&mut read_stream);
        let mut line = String::new();

        while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
            println!("{line}");
            line.clear();
        }
    });

    let mut stdin = io::stdin();

    // Lendo o input do client e enviando ao servidor
    tokio::spawn(async move {
        let mut reader = BufReader::new(&mut stdin).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            let _ = write_stream.write_all(line.as_bytes()).await;
        }
    });

    // Ctrl-C Quit
    loop {
        if tokio::signal::ctrl_c().await.is_ok() {
            println!("Encerrando");
            exit(0);
        }
    }
}
