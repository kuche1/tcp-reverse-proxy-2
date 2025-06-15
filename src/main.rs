// TODO
// make the connection encrypted

mod cmdline;
mod handle_client;
mod ip_translator;
mod log;

use crate::ip_translator::*;

// cargo add rustls_pemfile
// cargo add rustls
use rustls::{ServerConfig, ServerConnection, StreamOwned};
use std::fs::File;
use std::io::BufReader;
use std::net::TcpListener;
use std::process;
use std::sync::Arc;
use std::thread;

fn main() {
    //// parse args

    let args = cmdline::main();
    println!("{:?}", args);

    //// open encryption-related files

    let cert_file = &mut BufReader::new(match File::open(&args.cert_file) {
        Ok(v) => v,
        Err(e) => {
            log::err(
                &args.error_folder,
                &format!("could not open cert file `{}` -> {}", args.cert_file, e),
            );
            process::exit(1);
        }
    });

    let key_file = &mut BufReader::new(match File::open(&args.key_file) {
        Ok(v) => v,
        Err(e) => {
            log::err(
                &args.error_folder,
                &format!("could not open key file `{}` -> {}", args.key_file, e),
            );
            process::exit(1);
        }
    });

    // TODO get rid of the unwrap
    let cert_chain: Vec<_> = rustls_pemfile::certs(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .map(rustls::pki_types::CertificateDer::from)
        .collect();

    // TODO get rid of the unwrap
    let mut keys: Vec<_> = rustls_pemfile::pkcs8_private_keys(key_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let key = rustls::pki_types::PrivateKeyDer::from(keys.remove(0));

    // TODO get rid of the unwrap
    let tls_config = Arc::new(
        ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)
            .unwrap(),
    );

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

    //// create ip translator

    let mut ip_translator = IpTranslator::new();

    //// handle new connections

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(e) => {
                // eprintln!("connection failed -> {}", e);
                println!("connection failed -> {}", e);
                continue;
            }
        };

        let ip_original = match stream.peer_addr() {
            Ok(v) => v,
            Err(e) => {
                println!("could not get client address -> {}", e);
                continue;
            }
        };
        let ip_original = ip_original.ip();

        let ip_translated = ip_translator.translate(ip_original);

        println!(
            "new connection from {}; using translated ip {}",
            ip_original, ip_translated
        );

        let mut tls_conn = match ServerConnection::new(tls_config.clone()) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("could not create new TLS connection -> {}", e);
                continue;
            }
        };

        let mut tls_stream = StreamOwned::new(tls_conn, stream);

        thread::spawn(move || handle_client::main(tls_stream, ip_translated, args.remote_port));
    }
}
