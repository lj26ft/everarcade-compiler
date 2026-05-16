use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

pub type Hash256 = [u8; 32];

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CanonicalState {
    pub entries: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl CanonicalState {
    pub fn root(&self) -> Hash256 {
        let mut entry_hashes = Vec::with_capacity(self.entries.len() * 32);
        for (key, value) in &self.entries {
            let mut bytes = Vec::with_capacity(16 + key.len() + value.len());
            bytes.extend_from_slice(&(key.len() as u64).to_le_bytes());
            bytes.extend_from_slice(key);
            bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
            bytes.extend_from_slice(value);
            let h: [u8; 32] = Sha256::digest(bytes).into();
            entry_hashes.extend_from_slice(&h);
        }
        Sha256::digest(entry_hashes).into()
    }
}
