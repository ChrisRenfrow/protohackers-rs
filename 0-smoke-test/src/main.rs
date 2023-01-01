use std::io;

use clap::Parser;
use smoke_test::{cli::ServerArgs, server};

#[tokio::main]
async fn main() -> io::Result<()> {
    let port = ServerArgs::parse().port;

    server::run(port).await
}
