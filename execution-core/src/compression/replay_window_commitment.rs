use sha2::{Digest, Sha256};

use super::replay_summary::Hash;

pub fn replay_window_commitment(start: Hash, end: Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(start);
    hasher.update(end);
    hasher.finalize().into()
}
