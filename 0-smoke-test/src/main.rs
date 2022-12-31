use std::io;

use clap::Parser;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct ServerArgs {
    /// The port the server should bind to.
    #[arg(short, long, default_value_t = 4242)]
    port: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ServerArgs::parse();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;

    loop {
        let (sock, _) = listener.accept().await?;
        echo_socket(sock).await?;
    }
}

async fn echo_socket(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg = vec![0; 1024];

    loop {
        socket.readable().await?;

        match socket.try_read(&mut msg) {
            Ok(0) => {
                println!("Info: Read zero bytes, closing connection.");
                return Ok(());
            }
            Ok(n) => {
                let received = &msg[0..n];
                println!(
                    "Received message: {}\nEchoing...",
                    String::from_utf8(received.to_vec()).unwrap()
                );
                socket.write_all(received).await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("Info: No more bytes to read");
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
