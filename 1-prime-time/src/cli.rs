use clap::Parser;

#[derive(Parser, Debug)]
pub struct ServerArgs {
    pub port: u16,
}
