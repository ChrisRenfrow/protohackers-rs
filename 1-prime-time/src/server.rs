use std::{error::Error, io};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use crate::prime::is_prime;

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

impl IsPrimeResponse {
    fn new(prime: bool) -> Self {
        Self {
            method: "isPrime".to_string(),
            prime,
        }
    }
}

pub async fn run(port: u16) -> io::Result<()> {
    let listener = TcpListener::bind(("::", port)).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_is_prime(stream).await {
                eprintln!("{e}");
            }
        });
    }
}

async fn handle_is_prime(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut msg = vec![0; 1024];

    loop {
        stream.readable().await?;

        let now = Utc::now();

        match stream.try_read(&mut msg) {
            Ok(0) => {
                return Ok(());
            }
            Ok(n) => {
                let req: IsPrimeRequest = match serde_json::from_slice(&msg[0..n]) {
                    Ok(r) => r,
                    Err(e) => return Err(e.into()),
                };
                let res = match handle_is_prime_request(req) {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };

                stream
                    .write_all(serde_json::to_string(&res).unwrap().as_bytes())
                    .await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}

fn handle_is_prime_request(
    req: IsPrimeRequest,
) -> Result<IsPrimeResponse, Box<dyn std::error::Error>> {
    match &req.method[..] {
        "isPrime" => Ok(IsPrimeResponse::new(is_prime(req.number))),
        m => Err(format!("unsupported method: {}", m).into()),
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
