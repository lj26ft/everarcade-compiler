use crate::hashing::hash_bytes;

pub fn canonical_hash(bytes: &[u8]) -> String {
    hash_bytes(bytes)
}
