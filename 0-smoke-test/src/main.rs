use clap::Parser;
use smoke_test::{cli::ServerArgs, server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = ServerArgs::parse().port;

    server::run(port).await
}
