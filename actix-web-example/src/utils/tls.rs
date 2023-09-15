use std::env;
use std::path::Path;
use std::{fs::File, io::BufReader};

use rustls::{Certificate, PrivateKey, ServerConfig};
// use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys, ec_private_keys};

pub fn load_tls_config() -> ServerConfig {
    // init server config builder with safe defaults
    let mut cwd = env::current_dir().unwrap();
    cwd.push(Path::new("conf"));
    cwd.push(Path::new("server.pem"));
    let cert_path = cwd.to_str().unwrap();

    let mut cwd = env::current_dir().unwrap();
    cwd.push(Path::new("conf"));
    cwd.push(Path::new("server-key.pem"));
    let key_path = cwd.to_str().unwrap();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert_path.clone()).unwrap());
    let key_file = &mut BufReader::new(File::open(key_path.clone()).unwrap());

    // convert files to key/cert objects
    let cert_chain = rustls_pemfile::certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = rustls_pemfile::ec_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate EC private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
