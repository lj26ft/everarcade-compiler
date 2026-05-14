use execution_core::compression::replay_summary::ReplaySummary;

#[test]
fn replay_summary_holds_expected_fields() {
    let summary = ReplaySummary {
        epoch_index: 1,
        compressed_replay_root: [1; 32],
        aggregated_receipt_root: [2; 32],
        state_commitment_root: [3; 32],
    };
    assert_eq!(summary.epoch_index, 1);
}
