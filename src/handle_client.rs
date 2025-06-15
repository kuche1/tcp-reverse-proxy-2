use rustls::ServerConfig;
use rustls::ServerConnection;
use socket2::{Domain, SockAddr, Socket, Type}; // cargo add socket2
use std::io;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::sync::Arc;
use std::thread;

// pub fn main(
//     mut client_stream: TcpStream,
//     ip_translated: Ipv4Addr,
//     remote_port: u16,
//     tls_config: Arc<ServerConfig>,
// ) {
//     //// do TLS shits
//
//     // Create TLS connection
//     let mut server_conn = match ServerConnection::new(tls_config) {
//         Ok(conn) => conn,
//         Err(e) => {
//             eprintln!("TLS connection creation failed: {}", e);
//             return;
//         }
//     };
//
//     // Perform TLS handshake
//     let mut tls_stream = rustls::Stream::new(&mut server_conn, &mut client_stream);
//     if let Err(e) = server_conn.complete_io(&mut client_stream) {
//         eprintln!("TLS handshake failed: {}", e);
//         return;
//     }
//
//     // Connect to backend server (unencrypted)
//     // TODO hardcoded
//     let mut backend = match TcpStream::connect("127.0.0.1:32850") {
//         Ok(stream) => stream,
//         Err(e) => {
//             eprintln!("Backend connection failed: {}", e);
//             return;
//         }
//     };
//
//     // Bidirectional data transfer
//     let mut client_buffer = [0u8; 2048];
//     let mut backend_buffer = [0u8; 2048];
//
//     loop {
//         // Client -> Backend
//         match tls_stream.read(&mut client_buffer) {
//             Ok(0) => break, // EOF
//             Ok(n) => {
//                 if let Err(e) = backend.write_all(&client_buffer[..n]) {
//                     eprintln!("Backend write error: {}", e);
//                     break;
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Client read error: {}", e);
//                 break;
//             }
//         }
//
//         // Backend -> Client
//         match backend.read(&mut backend_buffer) {
//             Ok(0) => break, // EOF
//             Ok(n) => {
//                 if let Err(e) = tls_stream.write_all(&backend_buffer[..n]) {
//                     eprintln!("Client write error: {}", e);
//                     break;
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Backend read error: {}", e);
//                 break;
//             }
//         }
//
//         // Flush TLS stream
//         if let Err(e) = tls_stream.flush() {
//             eprintln!("Flush error: {}", e);
//             break;
//         }
//     }
//
//     println!("Connection closed");
//
//     //     ////
//     //
//     //     let local_addr = SocketAddrV4::new(ip_translated, 0);
//     //
//     //     let remote_ip = Ipv4Addr::new(127, 0, 0, 1);
//     //     let remote_addr = SocketAddrV4::new(remote_ip, remote_port);
//     //
//     //     let socket = match Socket::new(Domain::IPV4, Type::STREAM, None) {
//     //         Ok(v) => v,
//     //         Err(e) => {
//     //             eprintln!("could not create socket -> {}", e);
//     //             return;
//     //         }
//     //     };
//     //
//     //     if let Err(e) = socket.bind(&SockAddr::from(local_addr)) {
//     //         eprintln!("could not bind socket -> {}", e);
//     //         return;
//     //     }
//     //
//     //     if let Err(e) = socket.connect(&SockAddr::from(remote_addr)) {
//     //         eprintln!("could not connect to remote host {} -> {}", remote_addr, e);
//     //         return;
//     //     }
//     //
//     //     let mut remote_stream: TcpStream = socket.into();
//     //
//     //     //// forward data
//     //
//     //     let mut client_stream_clone = match client_stream.try_clone() {
//     //         Ok(s) => s,
//     //         Err(e) => {
//     //             eprintln!("could not clone client_stream -> {}", e);
//     //             return;
//     //         }
//     //     };
//     //     let mut remote_stream_clone = match remote_stream.try_clone() {
//     //         Ok(s) => s,
//     //         Err(e) => {
//     //             eprintln!("could not clone remote_stream -> {}", e);
//     //             return;
//     //         }
//     //     };
//     //
//     //     // forward: client -> remote
//     //     let client_to_remote = thread::spawn(move || {
//     //         let _ = io::copy(&mut client_stream, &mut remote_stream).ok();
//     //         let _ = remote_stream.shutdown(Shutdown::Write);
//     //     });
//     //
//     //     // forward: remote -> client
//     //     let remote_to_client = thread::spawn(move || {
//     //         let _ = io::copy(&mut remote_stream_clone, &mut client_stream_clone).ok();
//     //         let _ = client_stream_clone.shutdown(Shutdown::Write);
//     //     });
//     //
//     //     // wait for either direction to finish
//     //     let _ = client_to_remote.join();
//     //     let _ = remote_to_client.join();
// }

pub fn main(
    mut client_stream: TcpStream,
    ip_translated: Ipv4Addr,
    remote_port: u16,
    tls_config: Arc<ServerConfig>,
) {
    // Create TLS connection
    let mut server_conn = match ServerConnection::new(tls_config) {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("TLS connection creation failed: {}", e);
            return;
        }
    };

    // Perform TLS handshake FIRST without creating tls_stream
    if let Err(e) = server_conn.complete_io(&mut client_stream) {
        eprintln!("TLS handshake failed: {}", e);
        return;
    }

    // NOW create the TLS stream after handshake is complete
    let mut tls_stream = rustls::Stream::new(&mut server_conn, &mut client_stream);

    // Connect to backend server (unencrypted)
    let mut backend = match TcpStream::connect("127.0.0.1:32850") {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Backend connection failed: {}", e);
            return;
        }
    };

    // Bidirectional data transfer
    let mut client_buffer = [0u8; 2048];
    let mut backend_buffer = [0u8; 2048];

    loop {
        // Client -> Backend
        match tls_stream.read(&mut client_buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                if let Err(e) = backend.write_all(&client_buffer[..n]) {
                    eprintln!("Backend write error: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Client read error: {}", e);
                break;
            }
        }

        // Backend -> Client
        match backend.read(&mut backend_buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                if let Err(e) = tls_stream.write_all(&backend_buffer[..n]) {
                    eprintln!("Client write error: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Backend read error: {}", e);
                break;
            }
        }

        // Flush TLS stream
        if let Err(e) = tls_stream.flush() {
            eprintln!("Flush error: {}", e);
            break;
        }
    }

    println!("Connection closed");
}
