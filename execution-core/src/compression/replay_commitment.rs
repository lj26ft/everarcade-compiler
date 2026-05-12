use sha2::{Digest, Sha256};

use super::replay_summary::Hash;

pub fn replay_commitment(epoch_index: u64, replay_root: Hash, receipt_root: Hash, state_root: Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(epoch_index.to_le_bytes());
    hasher.update(replay_root);
    hasher.update(receipt_root);
    hasher.update(state_root);
    hasher.finalize().into()
}
