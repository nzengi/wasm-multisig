#[cfg(test)]
mod tests {
    use wasm_multisig::{sign_message, verify_signature}; // Fonksiyonları içe aktarıyoruz
    use secp256k1::Secp256k1;
    use secp256k1::{PublicKey, SecretKey};
    use rand::rngs::OsRng;
    use rand::RngCore;

    #[test]
    fn test_sign_and_verify_message() {
        let secp = Secp256k1::new();
        let mut rng = OsRng;

        // Rastgele bir secret key oluştur
        let mut secret_key_bytes = [0u8; 32];
        rng.fill_bytes(&mut secret_key_bytes);
        let secret_key = SecretKey::from_slice(&secret_key_bytes).expect("32-byte key expected");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        // Test edilecek bir mesaj
        let message = b"Test message for signing";

        // Mesajı imzala
        let signature = sign_message(message, &secret_key);

        // İmzayı doğrula
        assert!(verify_signature(message, &signature, &public_key));
    }
}
