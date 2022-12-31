use std::io;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

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
            Ok(0) => return Ok(()),
            Ok(n) => {
                msg.truncate(n);
                socket.write_all(&msg[..]).await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                eprintln!("{}", e);
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
