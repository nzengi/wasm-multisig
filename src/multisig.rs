use std::collections::HashMap;

pub struct MultisigWallet {
    pub signers: HashMap<String, bool>,  // List of signers and their signature status
    pub threshold: usize,                // Minimum number of signatures required
    pub signed_count: usize,             // Count of valid signatures so far
}

impl MultisigWallet {
    /// Creates a new MultisigWallet instance.
    ///
    /// # Arguments
    /// * `signers` - A list of signer addresses.
    /// * `threshold` - The minimum number of signatures required to approve a transaction.
    ///
    /// # Returns
    /// * A new instance of `MultisigWallet`.
    pub fn new(signers: Vec<String>, threshold: usize) -> Self {
        let mut signer_map = HashMap::new();
        for signer in signers {
            signer_map.insert(signer, false); // Initialize all signers with a 'false' status (not signed)
        }
        MultisigWallet {
            signers: signer_map,
            threshold,
            signed_count: 0, // Initially, no signers have signed
        }
    }

    /// Adds a signature to the wallet if the signer is valid and the signature is verified.
    ///
    /// # Arguments
    /// * `signer` - The signer's address.
    /// * `signature` - The cryptographic signature of the signer.
    ///
    /// # Returns
    /// * `Ok(())` if the signature is successfully added.
    /// * `Err` if the signature is invalid or the signer has already signed.
    pub fn sign(&mut self, signer: &String, signature: &str) -> Result<(), &'static str> {
        // First, verify the signature using an immutable reference
        let is_valid_signature = self.verify_signature(signer, signature);

        if is_valid_signature {
            // If the signature is valid, update the mutable state
            if let Some(signed) = self.signers.get_mut(signer) {
                if !*signed {
                    *signed = true;  // Mark the signer as having signed
                    self.signed_count += 1;  // Increment the count of valid signatures
                    return Ok(());
                }
            }
        } else {
            return Err("Invalid signature");
        }

        Err("Signer not found or already signed")
    }

    /// Verifies the signer's cryptographic signature.
    ///
    /// This is a placeholder function and should be replaced with actual signature verification logic.
    ///
    /// # Arguments
    /// * `signer` - The address of the signer.
    /// * `signature` - The signature to be verified.
    ///
    /// # Returns
    /// * `true` if the signature is valid.
    fn verify_signature(&self, _signer: &String, _signature: &str) -> bool {
        // Placeholder logic for signature verification.
        // Replace this with real cryptographic signature verification.
        true
    }

    /// Adds a new signer to the wallet.
    ///
    /// # Arguments
    /// * `signer` - The new signer's address to be added.
    pub fn add_signer(&mut self, signer: String) {
        self.signers.insert(signer, false);  // Add the new signer with a default 'not signed' status
    }

    /// Removes a signer from the wallet and updates the signature count if necessary.
    ///
    /// # Arguments
    /// * `signer` - The signer's address to be removed.
    pub fn remove_signer(&mut self, signer: &String) {
        if let Some(signed) = self.signers.remove(signer) {
            if signed {
                self.signed_count -= 1;  // Decrement the signature count if the signer had already signed
            }
        }
        // If removing the signer makes the signature count less than the threshold, panic
        if self.signers.len() < self.threshold {
            panic!("Not enough signers remaining to meet the threshold");
        }
    }

    /// Updates the signature threshold and checks if the current signature count is still valid.
    ///
    /// # Arguments
    /// * `new_threshold` - The new signature threshold.
    ///
    /// # Returns
    /// * `Ok(())` if the threshold is successfully updated.
    /// * `Err` if the threshold is invalid or if the current signatures do not meet the new threshold.
    pub fn update_threshold(&mut self, new_threshold: usize) -> Result<(), &'static str> {
        if new_threshold > self.signers.len() || new_threshold == 0 {
            return Err("Invalid threshold");
        }
        self.threshold = new_threshold;

        // Check if the current signature count meets the new threshold
        if self.signed_count < self.threshold {
            Err("Not enough valid signatures to meet the new threshold")
        } else {
            Ok(())
        }
    }

    /// Checks if the number of valid signatures meets or exceeds the threshold.
    ///
    /// # Returns
    /// * `true` if the required number of signatures is met.
    pub fn collect_signatures(&self) -> bool {
        self.signed_count >= self.threshold
    }
}
