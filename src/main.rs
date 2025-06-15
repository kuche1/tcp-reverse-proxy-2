mod cmdline;
mod handle_client;
mod ip_translator;
mod log;

use crate::ip_translator::*;

use std::net::TcpListener;
use std::process;
use std::thread;

fn main() {
    //// parse args

    let args = cmdline::main();
    println!("{:?}", args);

    //// create ip translator

    let mut ip_translator = IpTranslator::new();

    //// bind

    let addr = format!("0.0.0.0:{}", args.bind_port);

    println!("trying to bind to address {} -> working...", &addr);
    let listener = match TcpListener::bind(&addr) {
        Ok(v) => v,
        Err(e) => {
            log::err(
                &args.error_folder,
                &format!("could not bind to address `{}` -> {}", addr, e),
            );
            process::exit(1);
        }
    };
    println!("trying to bind to address {} -> done!", addr);

    //// handle new connections

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(e) => {
                // eprintln!("connection failed -> {}", e);
                println!("connection failed -> {}", e);
                continue;
            }
        };

        let ip_original = match stream.peer_addr() {
            Ok(v) => v,
            Err(e) => {
                println!("could not get client address -> {}", e);
                continue;
            }
        };
        let ip_original = ip_original.ip();

        println!("new connection from {}", ip_original);

        let ip_translated = ip_translator.translate(ip_original);

        println!("use translated ip {}", ip_translated);

        thread::spawn(move || handle_client::main(stream, ip_translated));
    }
}
