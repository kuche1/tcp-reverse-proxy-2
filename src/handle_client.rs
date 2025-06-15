use socket2::{Domain, SockAddr, Socket, Type}; // cargo add socket2
use std::io;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::thread;

pub fn main(mut client_stream: TcpStream, ip_translated: Ipv4Addr, remote_port: u16) {
    // // echo server
    // let mut buffer = [0; 512];
    // let n = stream.read(&mut buffer).unwrap(); // TODO get rid of the `unwrap`
    // stream.write_all(&buffer[..n]).unwrap(); // TODO get rid of the `unwrap`

    let local_addr = SocketAddrV4::new(ip_translated, 0);

    let remote_ip = Ipv4Addr::new(127, 0, 0, 1);
    let remote_addr = SocketAddrV4::new(remote_ip, remote_port);

    let socket = match Socket::new(Domain::IPV4, Type::STREAM, None) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("could not create socket -> {}", e);
            return;
        }
    };

    if let Err(e) = socket.bind(&SockAddr::from(local_addr)) {
        eprintln!("could not bind socket -> {}", e);
        return;
    }

    if let Err(e) = socket.connect(&SockAddr::from(remote_addr)) {
        eprintln!("could not connect to remote host {} -> {}", remote_addr, e);
        return;
    }

    let mut remote_stream: TcpStream = socket.into();

    ////// TODO generated code

    let mut client_stream_clone = match client_stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not clone client_stream -> {}", e);
            return;
        }
    };
    let mut remote_stream_clone = match remote_stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not clone remote_stream -> {}", e);
            return;
        }
    };

    // forward: client -> remote
    let client_to_remote = thread::spawn(move || {
        io::copy(&mut client_stream, &mut remote_stream).ok();
    });

    // forward: remote -> client
    let remote_to_client = thread::spawn(move || {
        io::copy(&mut remote_stream_clone, &mut client_stream_clone).ok();
    });

    // wait for either direction to finish
    let _ = client_to_remote.join();
    let _ = remote_to_client.join();
}
