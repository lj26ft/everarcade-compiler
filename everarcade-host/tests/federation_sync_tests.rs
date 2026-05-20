use everarcade_host::federation_transport::{sync, verification};
use execution_core::federation_runtime::bundle::ContinuityBundle;

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
    assert!(sync::sync_checkpoint(&sample(1), &sample(1)));
}
#[test]
fn test_journal_range_sync() {
    assert_eq!(sync::sync_journal_range(1, 3), vec![1, 2, 3]);
}
#[test]
fn test_continuity_bundle_exchange() {
    assert_eq!(sync::request_continuity_bundle(&sample(2)), sample(2));
}
#[test]
fn test_peer_replay_verification() {
    assert!(verification::verify_peer_checkpoint(&sample(3), &sample(3)));
}
