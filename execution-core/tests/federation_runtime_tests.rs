use execution_core::federation_runtime::{
    bundle::ContinuityBundle, divergence::detect_divergence, reconciliation::reconcile_peer,
    verification::verify_bundle_replay,
};

fn sample(seed: u8) -> ContinuityBundle {
    ContinuityBundle {
        state_root: [seed; 32],
        checkpoint_hash: [seed.wrapping_add(1); 32],
        journal_hash: [seed.wrapping_add(2); 32],
        receipt_hashes: vec![[seed.wrapping_add(3); 32]],
        execution_hashes: vec![[seed.wrapping_add(4); 32]],
        continuity_hash: [seed.wrapping_add(5); 32],
    }
}

#[test]
fn test_peer_checkpoint_sync() {
    assert_eq!(
        execution_core::federation_runtime::checkpoint_sync::sync_checkpoint(
            &sample(1),
            &sample(1)
        ),
        true
    );
}
#[test]
fn test_journal_range_sync() {
    assert_eq!(
        execution_core::federation_runtime::checkpoint_sync::sync_journal_range(3, 5),
        vec![3, 4, 5]
    );
}
#[test]
fn test_continuity_bundle_exchange() {
    assert_eq!(
        execution_core::federation_runtime::checkpoint_sync::request_continuity_bundle(&sample(2)),
        sample(2)
    );
}
#[test]
fn test_multi_node_replay_convergence() {
    assert!(verify_bundle_replay(&sample(3), &sample(3)));
}
#[test]
fn test_peer_replay_verification() {
    assert!(
        execution_core::federation_runtime::divergence::verify_peer_replay(&sample(4), &sample(4))
    );
}
#[test]
fn test_checkpoint_reconstruction_across_peers() {
    assert!(reconcile_peer(&sample(5), &sample(5)).is_some());
}
#[test]
fn test_detect_state_root_divergence() {
    let mut p = sample(6);
    p.state_root = [99; 32];
    assert!(detect_divergence(&sample(6), &p).is_some());
}
#[test]
fn test_detect_receipt_divergence() {
    let mut p = sample(7);
    p.receipt_hashes = vec![[98; 32]];
    assert!(detect_divergence(&sample(7), &p).is_some());
}
#[test]
fn test_detect_checkpoint_mismatch() {
    let mut p = sample(8);
    p.checkpoint_hash = [97; 32];
    assert!(detect_divergence(&sample(8), &p).is_some());
}
#[test]
fn test_reconcile_missing_journal_entries() {
    let mut p = sample(9);
    p.journal_hash = [96; 32];
    assert!(reconcile_peer(&sample(9), &p).is_none());
}
#[test]
fn test_reconcile_missing_checkpoints() {
    let mut p = sample(10);
    p.checkpoint_hash = [95; 32];
    assert!(reconcile_peer(&sample(10), &p).is_none());
}
#[test]
fn test_reconcile_peer_continuity() {
    assert!(reconcile_peer(&sample(11), &sample(11)).is_some());
}
