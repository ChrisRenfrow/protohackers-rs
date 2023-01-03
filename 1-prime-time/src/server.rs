use std::error::Error;

use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct IsPrimeRequest {
    method: String,
    number: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct IsPrimeResponse {
    method: String,
    prime: bool,
}

async fn run(port: u16) -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::bind(("::", port)).await?;

    loop {
        let (sock, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_is_prime(stream).await {
                eprintln!("{e}");
            }
        });
    }
}

async fn handle_is_prime(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut msg = vec![0; 1024];

    loop {
        stream.readable().await?;

        let now = Utc::now();

        match socket.try_read(&mut msg) {
            Ok(0) => {
                return Ok(());
            }
            Ok(n) => {
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_response() {
        let response = IsPrimeResponse {
            method: "isPrime".to_string(),
            prime: false,
        };
        let serialized = serde_json::to_string(&response).unwrap();
        let expected = "{\"method\":\"isPrime\",\"prime\":false}".to_string();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize_request() {
        let request = IsPrimeRequest {
            method: "isPrime".to_string(),
            number: 123,
        };
        let serialized = serde_json::to_string(&request).unwrap();
        assert_eq!(
            serialized,
            "{\"method\":\"isPrime\",\"number\":123}".to_string()
        );
    }

    #[test]
    fn serialize_request() {
        let request = "{\"method\":\"isPrime\",\"number\":123}";
        let deserialized: IsPrimeRequest = serde_json::from_str(request).unwrap();
        let expected = IsPrimeRequest {
            method: "isPrime".to_string(),
            number: 123,
        };
        assert_eq!(deserialized, expected);
    }
}
