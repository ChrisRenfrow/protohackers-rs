use std::io;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

pub async fn run(port: u32) -> io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    loop {
        let (sock, addr) = listener.accept().await?;
        println!("Accepted connection: {}", addr);
        tokio::spawn(async move { echo(sock).await });
    }
}

async fn echo(mut socket: TcpStream) {
    let mut msg = vec![0; 1024];

    loop {
        if let Err(e) = socket.readable().await {
            eprintln!("{}", e);
            return;
        }

        match socket.try_read(&mut msg) {
            Ok(0) => {
                println!("Info: Read zero bytes, closing connection.");
                return;
            }
            Ok(n) => {
                let received = &msg[0..n];
                println!(
                    "Received message: {}\nEchoing...",
                    String::from_utf8(received.to_vec()).unwrap()
                );
                if let Err(e) = socket.write_all(received).await {
                    eprintln!("{}", e);
                    return;
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!("Info: No more bytes to read");
                continue;
            }
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    }
}
