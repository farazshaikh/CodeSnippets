extern crate secp256k1;
extern crate tiny_keccak;

use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::Hasher;
use tiny_keccak::Keccak;

fn main() {
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let private_key: String = private_key[2..].into();
    let private_key_bytes = hex::decode(private_key.clone()).unwrap();
    let private_key = SecretKey::from_slice((&private_key_bytes[..]).into()).unwrap();

    // Initialize secp256k1 context
    let secp = Secp256k1::new();
    let sec1_pub_key_compressed = private_key.public_key(&secp).serialize();
    let sec1_pub_key_uncompressed = private_key.public_key(&secp).serialize_uncompressed();

    // Hash the uncompressed public key
    let mut public_key_hash = [0u8; 32];
    let mut keccak = Keccak::v256();
    keccak.update(&sec1_pub_key_compressed[1..]);
    keccak.finalize(&mut public_key_hash);

    // Take the last 20 bytes of the hash to obtain the Ethereum address
    let address = &public_key_hash[12..];
    println!(
        "Ethereum address: 0x{} 0x{sec1_pub_key_compressed:?} 0x{sec1_pub_key_uncompressed}",
        hex::encode(address)
    );
}
