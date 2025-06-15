use rustls::ServerConfig; // cargo add rustls
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
use rustls_pemfile::{certs, pkcs8_private_keys}; // cargo add rustls_pemfile
use socket2::{Domain, SockAddr, Socket, Type}; // cargo add socket2
use std::io;
use std::net::Shutdown;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::thread;
use std::{fs::File, io::BufReader, sync::Arc};
use tokio::net::TcpListener; // cargo add tokio --features full
use tokio_rustls::TlsAcceptor; // cargo add tokio-rustls

const HARDCODED_CERT_FILE: &str = "cert.pem";
const HARDCODED_KEY_FILE: &str = "privkey.pem";

pub async fn main(
    mut client_stream: std::net::TcpStream,
    ip_translated: Ipv4Addr,
    remote_port: u16,
) {
    //// FAWEIPDUCF*EYNCQ&F#YQC&H$FNW&C*FYG$W*GFT$W

    // Load certificate chain
    let cert_file = File::open(HARDCODED_CERT_FILE).expect("Cannot open certificate file");
    let cert_reader = &mut BufReader::new(cert_file);
    let cert_chain: Vec<CertificateDer<'static>> = certs(cert_reader)
        .map(|res| res.expect("Failed to parse certificate"))
        .collect();

    // Load private key (PKCS8)
    let key_file = File::open(HARDCODED_KEY_FILE).expect("Cannot open key file");
    let key_reader = &mut BufReader::new(key_file);
    let mut keys: Vec<PrivatePkcs8KeyDer<'static>> = pkcs8_private_keys(key_reader)
        .map(|res| res.expect("Failed to parse private key"))
        .collect();

    let private_key_pkcs8 = keys.pop().expect("No private key found");
    let private_key = PrivateKeyDer::Pkcs8(private_key_pkcs8);

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .expect("Invalid cert or key");

    let acceptor = TlsAcceptor::from(Arc::new(config));

    client_stream.set_nonblocking(true).unwrap();
    let client_stream = tokio::net::TcpStream::from_std(client_stream).unwrap();

    let client_stream = acceptor.accept(client_stream).await.unwrap();

    //// asdsafdsfgdsgfdsg

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

    let mut remote_stream: std::net::TcpStream = socket.into();
    let remote_stream = tokio::net::TcpStream::from_std(remote_stream).unwrap();

    let (mut ri, mut wi) = tokio::io::split(client_stream);
    let (mut ro, mut wo) = tokio::io::split(remote_stream);

    // Forward client -> remote
    let client_to_remote = tokio::spawn(async move { tokio::io::copy(&mut ri, &mut wo).await });

    // Forward remote -> client
    let remote_to_client = tokio::spawn(async move { tokio::io::copy(&mut ro, &mut wi).await });

    let (res1, res2) = tokio::join!(client_to_remote, remote_to_client);

    // Return error if any side failed
    // res1??;
    // res2??;

    //     //// forward data
    //
    //     let mut client_stream_clone = match client_stream.try_clone() {
    //         Ok(s) => s,
    //         Err(e) => {
    //             eprintln!("could not clone client_stream -> {}", e);
    //             return;
    //         }
    //     };
    //     let mut remote_stream_clone = match remote_stream.try_clone() {
    //         Ok(s) => s,
    //         Err(e) => {
    //             eprintln!("could not clone remote_stream -> {}", e);
    //             return;
    //         }
    //     };
    //
    //     // forward: client -> remote
    //     let client_to_remote = thread::spawn(move || {
    //         let _ = io::copy(&mut client_stream, &mut remote_stream).ok();
    //         let _ = remote_stream.shutdown(Shutdown::Write);
    //     });
    //
    //     // forward: remote -> client
    //     let remote_to_client = thread::spawn(move || {
    //         let _ = io::copy(&mut remote_stream_clone, &mut client_stream_clone).ok();
    //         let _ = client_stream_clone.shutdown(Shutdown::Write);
    //     });
    //
    //     // wait for either direction to finish
    //     let _ = client_to_remote.join();
    //     let _ = remote_to_client.join();
}
