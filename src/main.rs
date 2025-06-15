// TODO
// make the connection encrypted

mod cmdline;
mod handle_client;
mod ip_translator;
mod log;

use crate::ip_translator::*;

use std::net::TcpListener;
use std::process;
use std::thread;

// TODO load tls stuff
// cargo add rustls // cargo add rustls-pemfile // cargo add webpki-roots
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{Item, read_all};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

// TODO move elsewhere
fn load_tls_config(
    cert_path: &str,
    key_path: &str,
) -> Result<Arc<ServerConfig>, Box<dyn std::error::Error>> {
    // Load certificates
    let cert_file = File::open(cert_path)?;
    let mut cert_reader = BufReader::new(cert_file);

    let certs: Vec<CertificateDer> = read_all(&mut cert_reader)
        .filter_map(|item| match item {
            Ok(Item::X509Certificate(cert)) => Some(CertificateDer::from(cert)),
            _ => None,
        })
        .collect();

    if certs.is_empty() {
        return Err("No valid certificates found".into());
    }

    // Load private key
    let key_file = File::open(key_path)?;
    let mut key_reader = BufReader::new(key_file);

    let key = read_all(&mut key_reader)
        .find_map(|item| match item {
            Ok(Item::Pkcs8Key(k)) => Some(PrivateKeyDer::Pkcs8(k)),
            Ok(Item::Pkcs1Key(k)) => Some(PrivateKeyDer::Pkcs1(k)),
            Ok(Item::Sec1Key(k)) => Some(PrivateKeyDer::Sec1(k)),
            _ => None,
        })
        .ok_or("No valid private keys found")?;

    // Create TLS configuration
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| format!("TLS config error: {}", e))?;

    Ok(Arc::new(config))
}

fn main() {
    //// parse args

    let args = cmdline::main();
    println!("{:?}", args);

    //// load tls config

    // TODO hardcoded path
    let tls_config = load_tls_config("cert.pem", "privkey.pem")
        .expect("something wrong with loading the tls config");

    //// create ip translator

    let mut ip_translator = IpTranslator::new();

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

        let tls_config = tls_config.clone();

        thread::spawn(move || {
            handle_client::main(stream, ip_translated, args.remote_port, tls_config)
        });
    }
}
