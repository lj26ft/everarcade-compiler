use super::epoch::{ExecutionEpoch, Hash};
use super::epoch_root::compute_epoch_root;

pub fn build_epoch_boundary(
    epoch_index: u64,
    start_receipt: Hash,
    end_receipt: Hash,
    checkpoint_root: Hash,
    replay_root: Hash,
    receipt_root: Hash,
) -> ExecutionEpoch {
    let mut epoch = ExecutionEpoch {
        epoch_index,
        start_receipt,
        end_receipt,
        epoch_root: [0; 32],
        checkpoint_root,
        replay_root,
        receipt_root,
    };
    epoch.epoch_root = compute_epoch_root(&epoch);
    epoch
}
