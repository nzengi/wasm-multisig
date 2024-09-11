use std::collections::HashMap;

pub struct MultisigWallet {
    pub signers: HashMap<String, bool>, // İmza sahipleri
    pub threshold: usize,               // İmza eşiği
}

impl MultisigWallet {
    pub fn new(signers: Vec<String>, threshold: usize) -> Self {
        let mut signer_map = HashMap::new();
        for signer in signers {
            signer_map.insert(signer, false);
        }
        MultisigWallet {
            signers: signer_map,
            threshold,
        }
    }

    pub fn add_signer(&mut self, signer: String) {
        self.signers.insert(signer, false);
    }

    pub fn remove_signer(&mut self, signer: &String) {
        self.signers.remove(signer);
    }

    pub fn update_threshold(&mut self, new_threshold: usize) {
        self.threshold = new_threshold;
    }

    pub fn collect_signatures(&self) -> bool {
        let mut valid_signatures = 0;
        for &signed in self.signers.values() {
            if signed {
                valid_signatures += 1;
            }
        }
        valid_signatures >= self.threshold
    }
}
