// cargo run -- --server
// cargo run -- --message "Test message"
use clap::Parser;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::client::danger::{ServerCertVerifier, ServerCertVerified, HandshakeSignatureValid};
use rustls::DigitallySignedStruct;
use rustls::SignatureScheme;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run as server (if not set, runs as client)
    #[arg(short, long)]
    server: bool,

    /// Server address to connect to or listen on
    #[arg(short, long, default_value = "127.0.0.1:4433")]
    addr: String,

    /// Server name for TLS
    #[arg(short, long, default_value = "localhost")]
    name: String,

    /// Message to send (client only)
    #[arg(short, long, default_value = "Hello from QUIC client!")]
    message: String,
}

pub struct Config {
    pub server_addr: std::net::SocketAddr,
    pub server_name: String,
}

async fn run_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Create QUIC endpoint with same settings as proto.rs
    let cert = generate_self_signed_cert()?;
    let key = generate_private_key()?;

    let server_config = quinn::ServerConfig::with_single_cert(
        vec![rustls::Certificate(cert.to_vec())],
        rustls::PrivateKey(key),
    )?;

    let mut trans = quinn::TransportConfig::default();
    trans
        .keep_alive_interval(Some(std::time::Duration::from_secs(1)))
        .max_idle_timeout(Some(quinn::VarInt::from_u32(90_000u32).into()));

    let server_config = server_config.transport_config(Arc::new(trans));

    let sock = quinn::Endpoint::server(server_config, config.server_addr)?;
    println!("Server listening on {}", config.server_addr);

    while let Some(conn) = sock.accept().await {
        println!("New connection from {}", conn.remote_address());
        let connection = conn.await?;

        let (mut send, mut recv) = connection.open_bi().await?;
        println!("Opened bidirectional stream");

        // Read request
        let mut buffer = vec![0; 1024];
        let n = recv.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..n.unwrap_or(0)]);
        println!("Received request: {}", request);

        // Send response
        let response = format!("Server received: {}", request);
        send.write_all(response.as_bytes()).await?;
        println!("Sent response: {}", response);
    }

    Ok(())
}

async fn run_client(config: Config, message: String) -> Result<(), Box<dyn std::error::Error>> {
    // Create QUIC endpoint with same settings as proto.rs
    let mut sock = quinn::Endpoint::client(([0, 0, 0, 0], 0).into())?;

    // Configure TLS with same settings as proto.rs
    let mut rustls_config = rustls::ClientConfig::builder()
        .with_root_certificates(rustls::RootCertStore::empty())
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();

    // Set ALPN protocol
    rustls_config.alpn_protocols = vec![b"proton".to_vec()];

    let mut cfg = quinn::ClientConfig::new(Arc::new(rustls_config));
    let mut trans = quinn::TransportConfig::default();

    // Set same keep-alive and timeout settings
    trans
        .keep_alive_interval(Some(std::time::Duration::from_secs(1)))
        .max_idle_timeout(Some(quinn::VarInt::from_u32(90_000u32).into()));

    cfg.transport_config(Arc::new(trans));
    sock.set_default_client_config(cfg);

    // Connect to server
    let conn = sock
        .connect(config.server_addr, &config.server_name)?
        .await?;
    println!("Connected to server!");

    // Open a bidirectional stream
    let (mut send, mut recv) = conn.open_bi().await?;
    println!("Opened bidirectional stream");

    // Send message
    send.write_all(message.as_bytes()).await?;
    println!("Sent message: {}", message);

    // Read response
    let mut buffer = vec![0; 1024];
    let n = recv.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n.unwrap_or(0)]);
    println!("Received response: {}", response);

    Ok(())
}

#[derive(Debug)]
struct SkipServerVerification;

impl ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _scts: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
        ]
    }
}

fn generate_self_signed_cert() -> Result<CertificateDer<'static>, Box<dyn std::error::Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    Ok(CertificateDer::from(cert.serialize_der()?))
}

fn generate_private_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    Ok(cert.serialize_private_key_der())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = Config {
        server_addr: args.addr.parse()?,
        server_name: args.name,
    };

    if args.server {
        println!("Starting server...");
        run_server(config).await?;
    } else {
        println!("Starting client...");
        run_client(config, args.message).await?;
    }

    Ok(())
} 