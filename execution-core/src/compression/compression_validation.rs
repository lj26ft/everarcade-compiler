use super::{replay_commitment::replay_commitment, replay_summary::ReplaySummary};

pub fn validate_replay_summary(summary: &ReplaySummary, replay_root: [u8; 32]) -> bool {
    replay_commitment(
        summary.epoch_index,
        replay_root,
        summary.aggregated_receipt_root,
        summary.state_commitment_root,
    ) == summary.compressed_replay_root
}
