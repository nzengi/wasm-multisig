use secp256k1::{Secp256k1, Message, PublicKey, SecretKey};
use secp256k1::ecdsa::Signature;
use sha2::{Sha256, Digest}; // Include the sha2 crate to use SHA-256 hashing

/// Signs a message using a given secret key and returns the ECDSA signature.
///
/// # Arguments
/// * `message` - A byte slice representing the message to be signed.
/// * `secret_key` - A reference to the secret key that will be used to sign the message.
///
/// # Returns
/// * A Result containing the generated ECDSA signature, or an error if the signing process fails.
pub fn sign_message(message: &[u8], secret_key: &SecretKey) -> Result<Signature, secp256k1::Error> {
    let secp = Secp256k1::signing_only();

    // Hash the message using SHA-256 to create a 32-byte digest
    let hashed_message = Sha256::digest(message);
    
    // Create a Message object from the 32-byte hash digest
    let message = Message::from_slice(hashed_message.as_slice())?;
    
    // Sign the hashed message using the provided secret key
    Ok(secp.sign_ecdsa(&message, secret_key))
}

/// Verifies the signature of a message using the corresponding public key.
///
/// # Arguments
/// * `message` - A byte slice representing the original message.
/// * `signature` - The ECDSA signature to verify.
/// * `pubkey` - The public key corresponding to the secret key that signed the message.
///
/// # Returns
/// * A Result containing `true` if the signature is valid, or `false` if it is not.
///   In case of an error during verification, it returns the corresponding error.
pub fn verify_signature(message: &[u8], signature: &Signature, pubkey: &PublicKey) -> Result<bool, secp256k1::Error> {
    let secp = Secp256k1::verification_only();

    // Hash the message using SHA-256 to create a 32-byte digest
    let hashed_message = Sha256::digest(message);
    
    // Create a Message object from the 32-byte hash digest
    let message = Message::from_slice(hashed_message.as_slice())?;

    // Perform the ECDSA signature verification using the public key and the hashed message
    Ok(secp.verify_ecdsa(&message, signature, pubkey).is_ok())
}
