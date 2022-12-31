use std::io;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

pub async fn run(port: u32) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    loop {
        let (sock, _) = listener.accept().await?;
        echo(sock).await?;
    }
}

async fn echo(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
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
