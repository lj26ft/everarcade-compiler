use sha2::{Sha256, Digest};

pub fn hash_bytes(input: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hex::encode(hasher.finalize())
}

pub fn hash_state(state_bytes: &[u8]) -> String {
    hash_bytes(state_bytes)
}

pub fn hash_execution(input: &[u8]) -> String {
    hash_bytes(input)
}

pub fn hash_receipt(input: &[u8]) -> String {
    hash_bytes(input)
}
