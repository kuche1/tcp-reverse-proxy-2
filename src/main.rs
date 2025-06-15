mod cmdline;
mod fake_ip;
mod log;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::process;

// TODO get rid of this Result
fn main() -> std::io::Result<()> {
    //// parse args

    let args = cmdline::main();
    println!("{:?}", args);

    //// create fake ip giver

    let mut ip_generator = fake_ip::FakeIpGenerator::new();

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
        let ip = stream.peer_addr()?.ip();

        println!("new connection from {}", ip);

        let ip_faked = ip_generator.get(ip);

        println!("use fake ip {}", ip_faked);

        // echo server
        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer)?;
        stream.write_all(&buffer[..n])?;
    }

    //// return

    Ok(())
}
