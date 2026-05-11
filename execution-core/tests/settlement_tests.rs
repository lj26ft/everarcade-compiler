use execution_core::settlement::{anchor, finalize};

fn sample_checkpoint() -> execution_core::settlement::checkpoint::SettlementCheckpoint {
    finalize::finalize_checkpoint(
        "state-root".to_string(),
        "snapshot-hash".to_string(),
        "receipt-root".to_string(),
        "execution-root".to_string(),
        "consensus-hash".to_string(),
    )
}

#[test]
fn test_checkpoint_hash_stability() {
    let a = sample_checkpoint();
    let b = sample_checkpoint();
    assert_eq!(anchor::checkpoint_hash(&a), anchor::checkpoint_hash(&b));
}

#[test]
fn test_xrpl_anchor_payload() {
    let checkpoint = sample_checkpoint();
    let payload = anchor::xrpl_anchor_payload(&checkpoint);
    assert!(payload.starts_with("everarcade:settlement:"));
    assert_eq!(payload, anchor::xrpl_anchor_payload(&checkpoint));
}

#[test]
fn test_verifier_checkpoint_consensus() {
    let verifier_a = sample_checkpoint();
    let verifier_b = sample_checkpoint();
    assert_eq!(anchor::checkpoint_hash(&verifier_a), anchor::checkpoint_hash(&verifier_b));
}
