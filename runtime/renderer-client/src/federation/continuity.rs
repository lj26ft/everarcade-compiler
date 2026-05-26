use sha2::{Digest, Sha256};

pub fn continuity_hash(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for p in parts {
        hasher.update(p.as_bytes());
        hasher.update([0u8]);
    }
    format!("{:x}", hasher.finalize())
}
