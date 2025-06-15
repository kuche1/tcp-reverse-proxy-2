// use crate::log;

use clap::Parser; // cargo add clap --features derive

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Folder to write errors to
    #[arg(long)]
    pub error_folder: String,

    /// Port to bind to
    #[arg(long)]
    pub bind_port: u16,

    /// Server port
    #[arg(long)]
    pub remote_port: u16,

    /// Certificate file (example cert.pem)
    #[arg(long)]
    pub certfile: String,

    /// Key file (example privkey.pem)
    #[arg(long)]
    pub keyfile: String,
}

pub fn main() -> Args {
    Args::parse()
}
