use wasm_multisig::MultisigWallet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_multisig_wallet() {
        let signers = vec!["0xSigner1".to_string(), "0xSigner2".to_string()];
        let wallet = MultisigWallet::new(signers, 2);
        assert_eq!(wallet.threshold, 2);
        assert!(wallet.signers.contains_key("0xSigner1"));
    }
}
