# Multisig Wallet with ECDSA Signature and Verification

This project demonstrates how to generate, sign, and verify messages using the `secp256k1` elliptic curve digital signature algorithm (ECDSA) in Rust. It also includes a basic structure for a multisig wallet with support for multiple signatures, built using `secp256k1` and SHA-256.

## Features

- **ECDSA Signing**: Sign a message using a secret key (private key).
- **ECDSA Verification**: Verify the authenticity of a signed message using a corresponding public key.
- **Multisig Wallet**: A basic structure for managing multiple signers and tracking signature states.

## Dependencies

This project relies on the following Rust crates:

- `secp256k1`: Provides the ECDSA signing and verification functionality.
- `sha2`: Used for hashing the message using SHA-256.
- `rand`: Provides random number generation for creating secure secret keys.

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
secp256k1 = "0.21"
sha2 = "0.9"
rand = "0.8"
```
## Installation

```bash
git clone https://github.com/nzengi/wasm-multisig.git
cd wasm-multisig
cargo build
```

## Usage

Signing and Verifying Messages
The main functionality of this project is centered around signing and verifying messages. Below is a basic example of how to use this functionality in the main function.

```rust
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;

fn main() {
    // Initialize Secp256k1 context
    let secp = Secp256k1::new();
    let mut rng = OsRng;

    // Generate a random secret key
    let mut secret_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_key_bytes);
    let secret_key = SecretKey::from_slice(&secret_key_bytes).expect("32-byte key expected");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Message to sign
    let message = b"Example message to sign";

    // Sign the message
    let signature = sign_message(message, &secret_key);

    // Verify the signature
    let is_valid = verify_signature(message, &signature, &public_key);
    println!("Signature valid: {}", is_valid);
}
```

## Functions

1. sign_message(message: &[u8], secret_key: &SecretKey) -> Signature: This function takes a message and a secret key, hashes the message using SHA-256, and signs it using the ECDSA algorithm.

2. verify_signature(message: &[u8], signature: &Signature, pubkey: &PublicKey) -> bool: This function verifies if a signature is valid for the given message and public key by hashing the message and performing ECDSA verification.




