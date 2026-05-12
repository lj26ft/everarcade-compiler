use sha2::{Digest, Sha256};

use super::epoch::{ExecutionEpoch, Hash};

pub fn compute_epoch_root(epoch: &ExecutionEpoch) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(epoch.epoch_index.to_le_bytes());
    for root in [epoch.start_receipt, epoch.end_receipt, epoch.checkpoint_root, epoch.replay_root, epoch.receipt_root] {
        hasher.update(root);
    }
    hasher.finalize().into()
}
