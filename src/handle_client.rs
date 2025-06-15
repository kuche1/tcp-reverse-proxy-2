use rustls::ServerConfig;
use rustls::ServerConnection;
use socket2::{Domain, SockAddr, Socket, Type}; // cargo add socket2
use std::io;
use std::io::{Read, Write};
use std::net::Shutdown;
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

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

const NO_WORK_DONE_SLEEP: Duration = Duration::from_millis(10);
// TODO ideally we would use `epoll` or something like that

const BUFSIZE: usize = 1024 * 8;

fn stream_read<S: Read>(
    stream: &mut S,
    read_impossible: &mut bool,
    buffer: &mut [u8; BUFSIZE],
    buffer_start: &mut usize,
    buffer_end: &mut usize,
    any_work_done: &mut bool,
) {
    if !*read_impossible {
        if *buffer_end <= 0 {
            'scope: {
                let bytes_read = match stream.read(buffer) {
                    Ok(v) => v,
                    Err(e) => {
                        if (e.kind() == io::ErrorKind::WouldBlock)
                            || (e.kind() == io::ErrorKind::Interrupted)
                        {
                            // do nothing
                        } else {
                            *read_impossible = true;
                            eprintln!("stream read error -> {}", e);
                        }
                        break 'scope;
                    }
                };
                if bytes_read == 0 {
                    *read_impossible = true;
                    break 'scope;
                }

                *any_work_done = true;

                *buffer_start = 0;
                *buffer_end = bytes_read;
            }
        }
    }
}

fn stream_write<S: Write>(
    stream: &mut S,
    write_impossible: &mut bool,
    buffer: &mut [u8; BUFSIZE],
    buffer_start: &mut usize,
    buffer_end: &mut usize,
    any_work_done: &mut bool,
) {
    if !*write_impossible {
        if *buffer_end > 0 {
            'scope: {
                let bytes_written = match stream.write(&buffer[*buffer_start..*buffer_end]) {
                    Ok(v) => v,
                    Err(e) => {
                        if (e.kind() == io::ErrorKind::WouldBlock)
                            || (e.kind() == io::ErrorKind::Interrupted)
                        {
                            // do nothing
                        } else {
                            eprintln!("stream write error -> {}", e);
                            *write_impossible = true;
                        }
                        break 'scope;
                    }
                };
                if bytes_written == 0 {
                    *write_impossible = true;
                    break 'scope;
                }

                *any_work_done = true;

                *buffer_start = bytes_written;
                if buffer_start >= buffer_end {
                    *buffer_start = 0;
                    *buffer_end = 0;
                }
            }
        }
    }
}

// TODO also provide the client ip
//  then make a "title" that contains both, so that error printing is better
pub fn main(
    mut client_raw_stream: TcpStream,
    ip_translated: Ipv4Addr,
    remote_port: u16,
    tls_config: Arc<ServerConfig>,
    terminate_after_inactivity_ms: Option<u64>,
) {
    dbg!("in handle_client");

    //     //// timeout: client
    //
    //     let read_write_timeout_ms = match read_write_timeous_ms {
    //         None => None,
    //         Some(v) => Some(Duration::from_millis(v)),
    //     };
    //
    //     let _ = client_raw_stream.set_read_timeout(read_write_timeout_ms);
    //     let _ = client_raw_stream.set_write_timeout(read_write_timeout_ms);

    //// inactivity

    let terminate_after_inactivity = match terminate_after_inactivity_ms {
        None => None,
        Some(v) => Some(Duration::from_millis(v)),
    };

    //// tls

    dbg!("setup tls");

    let mut server_conn = match ServerConnection::new(tls_config) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("could not construct tls connection struct -> {}", e);
            return;
        }
    };

    if let Err(e) = server_conn.complete_io(&mut client_raw_stream) {
        eprintln!("tls handshake failed -> {}", e);
        return;
    }

    //// connect to remote

    dbg!("connect to remote");

    let local_addr = SocketAddrV4::new(ip_translated, 0);

    let remote_ip = Ipv4Addr::new(127, 0, 0, 1);
    let remote_addr = SocketAddrV4::new(remote_ip, remote_port);

    dbg!("new socket");

    let socket = match Socket::new(Domain::IPV4, Type::STREAM, None) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("could not create socket -> {}", e);
            return;
        }
    };

    dbg!("bind socket");

    if let Err(e) = socket.bind(&SockAddr::from(local_addr)) {
        eprintln!("could not bind socket -> {}", e);
        return;
    }

    dbg!("connect socket");

    // making the socket nonblocking before connecting is a bad idea
    if let Err(e) = socket.connect(&SockAddr::from(remote_addr)) {
        eprintln!("could not connect to remote host {} -> {}", remote_addr, e);
        return;
    }

    dbg!("convert into remote_stream");

    let mut remote_stream: TcpStream = socket.into();

    //     //// timeout: remote
    //
    //     let _ = remote_stream.set_read_timeout(read_write_timeout_ms);
    //     let _ = remote_stream.set_write_timeout(read_write_timeout_ms);

    //// make nonblocking

    dbg!("make nonblocking");

    if let Err(e) = client_raw_stream.set_nonblocking(true) {
        eprintln!("could not make client nonblocking -> {}", e);
        return;
    }

    if let Err(e) = remote_stream.set_nonblocking(true) {
        eprintln!("could not make remote nonblocking -> {}", e);
        return;
    }

    //// wrap client

    let mut client_stream = rustls::Stream::new(&mut server_conn, &mut client_raw_stream);

    //// forward data

    // TODO some code for
    //let _ = remote_stream.shutdown(Shutdown::Write);

    // TODO
    // it is possible to optimise the shutdown calls (remember: they need to be followed by flush (not sure if this is 100% true))

    let mut data_client_to_remote = [0u8; BUFSIZE];
    let mut data_client_to_remote_start = 0;
    let mut data_client_to_remote_end = 0;
    let mut client_read_impossible = false;
    let mut client_write_impossible = false;

    let mut data_remote_to_client = [0u8; BUFSIZE];
    let mut data_remote_to_client_start = 0;
    let mut data_remote_to_client_end = 0;
    let mut remote_read_impossible = false;
    let mut remote_write_impossible = false;

    let mut last_activity = Instant::now();

    loop {
        dbg!("loooooping");

        // break: if connection falls apart
        {
            //// this is more correct but less practical
            // let client_to_remote_impossible = client_read_impossible || remote_write_impossible;
            // let remote_to_client_impossible = remote_read_impossible || client_write_impossible;
            //
            // if client_to_remote_impossible && remote_to_client_impossible {
            //     break;
            // }
            //
            // TODO try `||`

            //// this is less correct but more practical
            // client(read) -> remote(write)
            // remote(read) -> client(write)
            if client_write_impossible {
                break;
            }
            if remote_read_impossible {
                break;
            }
            if remote_write_impossible {
                break;
            }
            if client_read_impossible {
                break;
            }
        }

        // TODO if we could find a way to get rid of those 2, that would be awesome (might be easier than it seems)
        if data_client_to_remote_end > 0 {
            if remote_write_impossible {
                data_client_to_remote_start = 0;
                data_client_to_remote_end = 0;
            }
        }
        if data_remote_to_client_end > 0 {
            if client_write_impossible {
                data_remote_to_client_start = 0;
                data_remote_to_client_end = 0;
            }
        }

        let mut any_work_done = false;

        stream_write(
            &mut remote_stream,
            &mut remote_write_impossible,
            &mut data_client_to_remote,
            &mut data_client_to_remote_start,
            &mut data_client_to_remote_end,
            &mut any_work_done,
        );

        stream_write(
            &mut client_stream,
            &mut client_write_impossible,
            &mut data_remote_to_client,
            &mut data_remote_to_client_start,
            &mut data_remote_to_client_end,
            &mut any_work_done,
        );

        stream_read(
            &mut client_stream,
            &mut client_read_impossible,
            &mut data_client_to_remote,
            &mut data_client_to_remote_start,
            &mut data_client_to_remote_end,
            &mut any_work_done,
        );

        stream_read(
            &mut remote_stream,
            &mut remote_read_impossible,
            &mut data_remote_to_client,
            &mut data_remote_to_client_start,
            &mut data_remote_to_client_end,
            &mut any_work_done,
        );

        if any_work_done {
            dbg!("did some work");
            last_activity = Instant::now();
        } else {
            dbg!("no work done");
            // break: in case of inactivity
            if Some(last_activity.elapsed()) >= terminate_after_inactivity {
                // this is actually not the greatest since it is possible that
                // a very long blocking took place (see where `any_work_done` is being set)
                break;
            }

            thread::sleep(NO_WORK_DONE_SLEEP);
        }
    }

    dbg!("main loop broken");

    //// TODO the flushing is not great, let's make sure all is flushed

    //// flush: remote -> client

    while data_remote_to_client_end > 0 {
        let mut _any_work_done: bool = false;
        stream_write(
            &mut client_stream,
            &mut client_write_impossible,
            &mut data_remote_to_client,
            &mut data_remote_to_client_start,
            &mut data_remote_to_client_end,
            &mut _any_work_done,
        );
    }

    if let Err(e) = client_stream.flush() {
        eprintln!("could not flush client -> {}", e);
    }

    //// flush: client -> remote

    while data_client_to_remote_end > 0 {
        let mut _any_work_done: bool = false;
        stream_write(
            &mut remote_stream,
            &mut remote_write_impossible,
            &mut data_client_to_remote,
            &mut data_client_to_remote_start,
            &mut data_client_to_remote_end,
            &mut _any_work_done,
        );
    }

    if let Err(e) = remote_stream.flush() {
        eprintln!("could not flush remote -> {}", e);
    }

    //// shutdown

    server_conn.send_close_notify();
    if let Err(e) = server_conn.complete_io(&mut client_raw_stream) {
        eprintln!("failed to send_close_notify client -> {}", e);
    }

    if let Err(e) = remote_stream.shutdown(Shutdown::Both) {
        eprintln!("remote shutdown error -> {}", e);
    }

    if let Err(e) = client_raw_stream.shutdown(Shutdown::Both) {
        eprintln!("client shutdown error -> {}", e);
    }

    println!("connection closed");
}
