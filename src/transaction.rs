pub struct Transaction {
    pub nonce: u64,
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn new(data: Vec<u8>, nonce: u64) -> Self {
        Transaction { data, nonce }
    }

    pub fn is_valid_nonce(&self, current_nonce: u64) -> bool {
        self.nonce == current_nonce
    }
}
