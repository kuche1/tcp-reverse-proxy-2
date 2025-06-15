use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

pub fn main(mut stream: TcpStream, ip_translated: Ipv4Addr, remote_port: u16) {
    // // echo server
    // let mut buffer = [0; 512];
    // let n = stream.read(&mut buffer).unwrap(); // TODO get rid of the `unwrap`
    // stream.write_all(&buffer[..n]).unwrap(); // TODO get rid of the `unwrap`

    let local_addr = SocketAddrV4::new(ip_translated, 0);

    let remote_ip = Ipv4Addr::new(127, 0, 0, 1);
    let remote_addr = SocketAddrV4::new(remote_ip, remote_port);
}
