// cargo add rustls // cargo add rustls-pemfile // cargo add webpki-roots
use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{Item, read_all};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub fn main(
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
