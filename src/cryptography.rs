use secp256k1::{Secp256k1, Message, PublicKey, SecretKey};
use secp256k1::ecdsa::Signature;
use sha2::{Sha256, Digest}; // SHA-256 kullanmak için sha2 crate'ini ekleyin

// Mesajı imzalama fonksiyonu
pub fn sign_message(message: &[u8], secret_key: &SecretKey) -> Signature {
    let secp = Secp256k1::signing_only();

    // Mesajı hash'leyin ve 32-byte uzunluğunda bir Message yapısı oluşturun
    let hashed_message = Sha256::digest(message);
    let message = Message::from_slice(hashed_message.as_slice()).expect("32-byte message expected");
    
    secp.sign_ecdsa(&message, secret_key)
}

// İmzayı doğrulama fonksiyonu
pub fn verify_signature(message: &[u8], signature: &Signature, pubkey: &PublicKey) -> bool {
    let secp = Secp256k1::verification_only();

    // Mesajı hash'leyin ve 32-byte uzunluğunda bir Message yapısı oluşturun
    let hashed_message = Sha256::digest(message);
    let message = Message::from_slice(hashed_message.as_slice()).expect("32-byte message expected");
    
    secp.verify_ecdsa(&message, signature, pubkey).is_ok()
}
