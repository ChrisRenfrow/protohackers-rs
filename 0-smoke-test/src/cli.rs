use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct ServerArgs {
    /// The port the server should bind to.
    #[arg(short, long, default_value_t = 4242)]
    pub port: u16,
}
