use super::{anchor, checkpoint::SettlementCheckpoint};

pub fn verify_checkpoint_hash(checkpoint: &SettlementCheckpoint, expected_hash: &str) -> bool {
    anchor::checkpoint_hash(checkpoint) == expected_hash
}
