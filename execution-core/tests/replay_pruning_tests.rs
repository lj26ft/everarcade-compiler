use execution_core::pruning::{
    pruning_policy::PruningPolicy, replay_pruning::pruneable_epochs,
    retention_window::RetentionWindow,
};

#[test]
fn pruning_respects_retention_window() {
    let policy = PruningPolicy {
        retention: RetentionWindow {
            keep_last_epochs: 2,
        },
        require_checkpoint: true,
        require_proof_commitment: true,
    };
    assert_eq!(pruneable_epochs(5, &policy), vec![0, 1, 2]);
}
