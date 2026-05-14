use super::checkpoint_delta::CheckpointDelta;

pub fn validate_checkpoint_chain(delta: &CheckpointDelta) -> bool {
    delta.missing_receipts <= u64::MAX / 2
}
