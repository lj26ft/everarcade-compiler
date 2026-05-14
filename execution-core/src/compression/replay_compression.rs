use super::{
    receipt_aggregation::aggregate_receipts,
    replay_commitment::replay_commitment,
    replay_summary::{Hash, ReplaySummary},
};

pub fn compress_replay(
    epoch_index: u64,
    replay_root: Hash,
    receipts: Vec<Hash>,
    state_commitment_root: Hash,
) -> ReplaySummary {
    let aggregated_receipt_root = aggregate_receipts(receipts);
    let compressed_replay_root = replay_commitment(
        epoch_index,
        replay_root,
        aggregated_receipt_root,
        state_commitment_root,
    );
    ReplaySummary {
        epoch_index,
        compressed_replay_root,
        aggregated_receipt_root,
        state_commitment_root,
    }
}
