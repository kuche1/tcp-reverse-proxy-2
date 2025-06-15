mod cmdline;
mod log;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::process;

// TODO get rid of this Result
fn main() -> std::io::Result<()> {
    let args = cmdline::main();
    println!("{:?}", args);

    let addr = format!("0.0.0.0:{}", args.bind_port);

    println!("trying to bind to address {} -> working...", &addr);
    let listener = match TcpListener::bind(&addr) {
        Ok(v) => v,
        Err(e) => {
            log::err(
                &args.error_folder,
                &format!("could not bind to address `{}` -> `{}`", addr, e),
            );
            process::exit(1);
        }
    };
    println!("trying to bind to address {} -> done!", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                // echo server
                let mut buffer = [0; 512];
                let n = stream.read(&mut buffer)?;
                stream.write_all(&buffer[..n])?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
