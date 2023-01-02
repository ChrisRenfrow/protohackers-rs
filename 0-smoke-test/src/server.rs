use std::{error::Error, io};

use chrono::prelude::*;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

pub async fn run(port: u16) -> io::Result<()> {
    let now = Utc::now();
    let listener = TcpListener::bind(("::", port)).await?;

    loop {
        let (sock, addr) = listener.accept().await?;
        println!("{}: Accepted connection: {}", now, addr);
        tokio::spawn(async move {
            if let Err(e) = echo(sock).await {
                eprintln!("{e}");
            }
        });
    }
}

async fn echo(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut msg = vec![0; 1024];

    loop {
        socket.readable().await?;

        let now = Utc::now();

        match socket.try_read(&mut msg) {
            Ok(0) => {
                println!("{}: Info: Read zero bytes, exiting...", now);
                return Ok(());
            }
            Ok(n) => {
                let received = &msg[0..n];
                println!(
                    "{}: Received message: {}\nEchoing...",
                    now,
                    String::from_utf8(received.to_vec()).unwrap()
                );
                socket.write_all(received).await?;
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                println!(
                    "{}: Info: No more bytes to read, shutting-down connection...",
                    now
                );
                socket.shutdown().await?;
                println!("{}: Info: Bye!", Utc::now());
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
