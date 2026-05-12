use execution_core::pruning::{pruning_policy::PruningPolicy, pruning_validation::validate_pruning_policy, retention_window::RetentionWindow};

#[test]
fn pruning_requires_replay_safety_artifacts() {
    let policy = PruningPolicy { retention: RetentionWindow { keep_last_epochs: 10 }, require_checkpoint: true, require_proof_commitment: true };
    assert!(validate_pruning_policy(&policy));
}
