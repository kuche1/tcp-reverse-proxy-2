mod cmdline;
mod handle_client;
mod ip_translator;
mod log;

use crate::ip_translator::*;

// use std::io::{Read, Write};
use std::net::TcpListener;
use std::process;

// TODO get rid of this Result
fn main() -> std::io::Result<()> {
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
        let mut stream = match stream {
            Ok(v) => v,
            Err(e) => {
                // eprintln!("connection failed -> {}", e);
                println!("connection failed -> {}", e);
                continue;
            }
        };

        // TODO do not use `?`
        let ip_original = stream.peer_addr()?.ip();

        println!("new connection from {}", ip_original);

        let ip_translated = ip_translator.translate(ip_original);

        println!("use translated ip {}", ip_translated);

        handle_client::main(stream, ip_translated);

        // // echo server
        // let mut buffer = [0; 512];
        // let n = stream.read(&mut buffer)?;
        // stream.write_all(&buffer[..n])?;
    }

    //// return

    Ok(())
}
