mod cmdline;
mod log;

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let args = cmdline::main();
    println!("{:?}", args);

    let addr = format!("0.0.0.0:{}", args.bind_port);

    println!("trying to bind to address {} -> working...", &addr);
    let listener = TcpListener::bind(&addr)?;
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
