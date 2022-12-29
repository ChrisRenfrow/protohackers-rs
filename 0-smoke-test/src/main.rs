use tokio::net::{TcpListener, TcpStream};

use std::{error::Error, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}

async fn process_socket(socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut msg = vec![0; 1024];
    
    loop {
        socket.readable().await?;

        match socket.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    println!("Got: {}", String::from_utf8(msg).unwrap());
    
    Ok(())
}
