use prime_time::server;

use serde::{Deserialize, Serialize};

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

mod server {
    use std::{error::Error, net::TcpStream};

    async fn run(port: u16) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(("::", port)).await?;

        loop {
            let (sock, addr) = listener.accept().await?;
            tokio::spawn(async move {
                if let Err(e) = handle_is_prime(stream).await {
                    eprintln!("{e}");
                }
            });
        }
    }

    async fn handle_is_prime(stream: TcpStream) -> Result<(), Box<dyn Error>> {}
}

fn main() {}

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

    #[test]
    fn prime() {
        let mut primes = vec![];
        for n in 0..20 {
            if is_prime(n) {
                primes.push(n);
            }
        }
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
