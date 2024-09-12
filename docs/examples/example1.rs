use secp256k1::{Secp256k1, Message, PublicKey, SecretKey};
use secp256k1::ecdsa::Signature;
use sha2::{Sha256, Digest}; // Include sha2 crate for SHA-256 hashing
use rand::rngs::OsRng;
use rand::RngCore;

/// Signs a message using the provided secret key and returns the ECDSA signature.
///
/// # Arguments
/// * `message` - A byte slice representing the message to be signed.
/// * `secret_key` - The secret key used to sign the message.
///
/// # Returns
/// * An `ecdsa::Signature` representing the signed message.
pub fn sign_message(message: &[u8], secret_key: &SecretKey) -> Signature {
    let secp = Secp256k1::signing_only();

    // Hash the message using SHA-256 to produce a 32-byte digest
    let hashed_message = Sha256::digest(message);
    let message = Message::from_slice(&hashed_message).expect("32-byte message expected");
    
    // Sign the hashed message using the provided secret key
    secp.sign_ecdsa(&message, secret_key)
}

/// Verifies the signature of a message using the corresponding public key.
///
/// # Arguments
/// * `message` - A byte slice representing the original message.
/// * `signature` - The ECDSA signature to verify.
/// * `pubkey` - The public key associated with the signer.
///
/// # Returns
/// * `true` if the signature is valid, otherwise `false`.
pub fn verify_signature(message: &[u8], signature: &Signature, pubkey: &PublicKey) -> bool {
    let secp = Secp256k1::verification_only();

    // Hash the message using SHA-256 to produce a 32-byte digest
    let hashed_message = Sha256::digest(message);
    let message = Message::from_slice(&hashed_message).expect("32-byte message expected");
    
    // Verify the ECDSA signature using the public key and hashed message
    secp.verify_ecdsa(&message, signature, pubkey).is_ok()
}

fn main() {
    // Generate a key pair using Secp256k1
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // Generate a random secret key
    let mut secret_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_key_bytes);
    let secret_key = SecretKey::from_slice(&secret_key_bytes).expect("32-byte key expected");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Message to be signed
    let message = b"Example message to sign";

    // Sign the message
    let signature = sign_message(message, &secret_key);

    // Verify the signature
    let is_valid = verify_signature(message, &signature, &public_key);
    println!("Signature valid: {}", is_valid);
}
