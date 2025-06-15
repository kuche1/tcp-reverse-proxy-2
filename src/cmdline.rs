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
    //// TODO delete all of this
    //     /// At what hour the restart is going to occur, for example 15 for 15:00
    //     #[arg(short, long, default_value_t = 4)]
    //     pub restart_at: u8,
    //
    //     /// Time to sleep if restart time has not been reached
    //     #[arg(long, default_value_t = 3000)] // 3000sec = 50min
    //     pub check_time_sleep_sec: u64,
    //
    //     /// Stop "all" services with this regex before restarting
    //     #[arg(long)]
    //     pub services_regex: String,
    //
    //     /// Exclude this service from being restarted
    //     #[arg(long)]
    //     pub service_exception: String,
    //
    //     /// IP of the backup server
    //     #[arg(long)]
    //     pub backup_server_ip: String,
    //
    //     /// User on the backup server
    //     #[arg(long)]
    //     pub backup_server_user: String,
    //
    //     /// Update the server, as if it is debian-based
    //     #[arg(long, default_value_t = false)]
    //     pub update_server_debian: bool,
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
