use crate::integrity::artifact_hash::hash_bytes;

use super::checkpoint_export::TransferCheckpoint;

pub fn validate_checkpoint_root(checkpoint: &TransferCheckpoint) -> bool {
    hash_bytes(&checkpoint.state_bytes) == checkpoint.checkpoint_root
}

pub fn lineage_is_continuous(previous: [u8; 32], next_parent: [u8; 32]) -> bool {
    previous == next_parent
}
