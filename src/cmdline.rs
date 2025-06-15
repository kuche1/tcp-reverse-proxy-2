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
}

pub fn main() -> Args {
    let args = Args::parse();

    //     if args.restart_at >= 24 {
    //         log::err(
    //             &args.error_folder,
    //             &format!(
    //                 "invalid hour `{}`, needs to be less than 24",
    //                 args.restart_at
    //             ),
    //         );
    //         panic!();
    //     }
    //
    //     if args.restart_at == 0 {
    //         log::err(
    //             &args.error_folder,
    //             "restarting at midnight is not supported, sorry",
    //         );
    //         panic!();
    //     }

    args
}
