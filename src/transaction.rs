/// A struct representing a transaction with a nonce and associated data.
/// The `Transaction` structure is used to manage operations with specific data and unique nonces.
pub struct Transaction {
    pub nonce: u64,      // A unique identifier for the transaction to prevent replay attacks
    pub data: Vec<u8>,   // The transaction's associated data (with a limit on size)
}

impl Transaction {
    /// Creates a new `Transaction` with the given data and nonce.
    /// The data size is limited to 1024 bytes to ensure the transaction payload is reasonable.
    ///
    /// # Arguments
    /// * `data` - A vector of bytes representing the transaction's data.
    /// * `nonce` - A unique number (nonce) to differentiate this transaction from others.
    ///
    /// # Panics
    /// Panics if the `data` length exceeds 1024 bytes.
    pub fn new(data: Vec<u8>, nonce: u64) -> Self {
        assert!(data.len() < 1024, "Data too large");  // Ensure the data size is within the limit
        Transaction { data, nonce }
    }

    /// Validates whether the provided nonce matches the transaction's nonce.
    ///
    /// # Arguments
    /// * `current_nonce` - The nonce to be compared with the transaction's nonce.
    ///
    /// # Returns
    /// * `Ok(())` if the nonce is valid.
    /// * `Err(&'static str)` if the nonce does not match, with an error message.
    pub fn is_valid_nonce(&self, current_nonce: u64) -> Result<(), &'static str> {
        if self.nonce != current_nonce {
            Err("Invalid nonce")  // Return an error if the nonces do not match
        } else {
            Ok(())  // Return Ok if the nonce matches
        }
    }
}
