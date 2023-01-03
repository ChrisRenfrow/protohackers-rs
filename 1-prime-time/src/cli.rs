use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct ServerArgs {
    /// The port the server should bind to
    #[arg(short, long)]
    pub port: u16,
}
