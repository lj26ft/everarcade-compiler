use everarcade_host::federation_transport::bundle_exchange;
use execution_core::federation_runtime::{
    reconcile_peer_state, sync_peer, ContinuityBundle, TopologyStateEngine,
};
use execution_core::federation_runtime::replay_verification::verify_peer_replay;

fn bundle(seed: u8) -> ContinuityBundle {
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
fn test_multi_node_global_convergence() {
    let node_a = bundle(7);
    let node_b = bundle(7);
    let node_c = bundle(7);
    let mut topology = TopologyStateEngine::default();

    sync_peer(&mut topology, [0xAA; 32], &node_a, &node_b).expect("sync b");
    sync_peer(&mut topology, [0xCC; 32], &node_b, &node_c).expect("sync c");

    verify_peer_replay(&node_a, &node_b).expect("a-b");
    verify_peer_replay(&node_b, &node_c).expect("b-c");
    assert_eq!(
        reconcile_peer_state(&node_a, &node_c)
            .expect("reconcile")
            .state_root,
        node_a.state_root
    );
}

#[test]
fn test_detect_checkpoint_divergence() {
    let a = bundle(1);
    let mut b = bundle(1);
    b.checkpoint_hash = [9; 32];
    assert!(reconcile_peer_state(&a, &b).is_err());
}

#[test]
fn test_detect_receipt_divergence() {
    let a = bundle(1);
    let mut b = bundle(1);
    b.receipt_hashes = vec![[8; 32]];
    assert!(verify_peer_replay(&a, &b).is_err());
}

#[test]
fn test_detect_execution_hash_divergence() {
    let a = bundle(1);
    let mut b = bundle(1);
    b.execution_hashes = vec![[8; 32]];
    assert!(verify_peer_replay(&a, &b).is_err());
}

#[test]
fn test_detect_state_root_divergence() {
    let a = bundle(1);
    let mut b = bundle(1);
    b.state_root = [3; 32];
    assert!(verify_peer_replay(&a, &b).is_err());
}

#[test]
fn test_reconcile_missing_journal_entries() {
    let peer = bundle(4);
    let journal = bundle_exchange::request_journal_bundle(&peer).expect("journal");
    assert_eq!(journal.journal_hash, peer.journal_hash);
}

#[test]
fn test_reconcile_missing_checkpoints() {
    let peer = bundle(4);
    let checkpoint = bundle_exchange::request_checkpoint_bundle(&peer).expect("checkpoint");
    assert_eq!(checkpoint.checkpoint_hash, peer.checkpoint_hash);
}

#[test]
fn test_recover_peer_after_restart() {
    let peer = bundle(4);
    let replay = bundle_exchange::request_replay_proof_bundle(&peer).expect("proof");
    assert_eq!(replay.state_root, peer.state_root);
}

#[test]
fn test_recover_peer_after_partial_sync() {
    let peer = bundle(4);
    let receipt = bundle_exchange::request_receipt_bundle(&peer).expect("receipt");
    assert_eq!(receipt.execution_hashes, peer.execution_hashes);
}
