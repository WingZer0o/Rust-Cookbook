use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, pkcs8::{EncodePrivateKey, DecodePrivateKey}, RsaPublicKey, PublicKeyParts, PublicKey, pkcs1::{EncodeRsaPublicKey, DecodeRsaPrivateKey}};

fn main() {
    let testing: String = generate_rsa4096_private_key();
    let public_key: String = rsa4096_testing(&testing);
    println!("{}", public_key);
}

fn generate_rsa4096_private_key() -> String {
    let mut rng = OsRng;
    let bits = 4096;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    return priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::CRLF).unwrap().to_string();
}

fn rsa4096_testing(pem_string: &str) -> String {
    let mut rng: OsRng = OsRng;
    let bits: usize = 4096;
    let priv_key = RsaPrivateKey::from_pkcs8_pem(pem_string).unwrap();
    let pub_key = priv_key.to_public_key();
    let encrypted = pub_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).unwrap().to_string();
    return encrypted;
}
