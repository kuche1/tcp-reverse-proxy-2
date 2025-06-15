use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};

pub fn main(mut stream: TcpStream, ip_translated: Ipv4Addr, server_port: u16) {
    // echo server
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer).unwrap(); // TODO get rid of the `unwrap`
    stream.write_all(&buffer[..n]).unwrap(); // TODO get rid of the `unwrap`
}
