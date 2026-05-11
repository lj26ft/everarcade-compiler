use execution_core::finality::canonical::CanonicalHistory;
use execution_core::finality::challenge::ExecutionChallenge;
use execution_core::finality::checkpoint::FinalityCheckpoint;
use execution_core::finality::finality::{FinalityState, FinalityTracker};
use execution_core::finality::quorum::QuorumConfig;
use execution_core::finality::receipt_consensus::resolve_receipt_consensus;
use execution_core::finality::settlement_finality::create_settlement_anchor;
use execution_core::finality::voting::VerifierVote;

fn vote(verifier: &str, receipt: &str, epoch_id: u64) -> VerifierVote {
    VerifierVote {
        verifier_id: verifier.to_string(),
        receipt_hash: receipt.to_string(),
        execution_root: "exec-root".to_string(),
        snapshot_hash: "snap-root".to_string(),
        epoch_id,
    }
}

#[test]
fn test_verifier_quorum_consensus() {
    let quorum = QuorumConfig::default();
    assert!(quorum.replay_quorum_reached(2, 3));
}

#[test]
fn test_invalid_receipt_challenge() {
    let challenge = ExecutionChallenge {
        challenger: "v1".into(),
        challenged_receipt_hash: "bad".into(),
        canonical_receipt_hash: "good".into(),
    };
    assert!(challenge.is_valid());
}

#[test]
fn test_finality_transition() {
    let mut tracker = FinalityTracker::new();
    tracker.begin_replay();
    tracker.accept(10);
    assert_eq!(tracker.state, FinalityState::Accepted);
    assert!(tracker.finalize_if_window_elapsed(20, 10));
    assert_eq!(tracker.state, FinalityState::Finalized);
}

#[test]
fn test_challenge_window_expiration() {
    let mut tracker = FinalityTracker::new();
    tracker.accept(5);
    assert!(!tracker.finalize_if_window_elapsed(14, 10));
    assert!(tracker.finalize_if_window_elapsed(15, 10));
}

#[test]
fn test_cross_verifier_replay_consistency() {
    let votes = vec![vote("v1", "r1", 1), vote("v2", "r1", 1), vote("v3", "r1", 1)];
    let result = resolve_receipt_consensus(&votes).expect("consensus");
    assert_eq!(result.canonical_receipt_hash, "r1");
    assert_eq!(result.agreeing_verifiers.len(), 3);
}

#[test]
fn test_checkpoint_finalization() {
    let checkpoint = FinalityCheckpoint {
        execution_root: "e".into(),
        receipt_root: "r".into(),
        snapshot_root: "s".into(),
        epoch_id: 7,
        verifier_quorum_proof: "2/3".into(),
    };

    let mut history = CanonicalHistory::default();
    history.append_finalized(checkpoint.clone());
    assert_eq!(history.latest(), Some(&checkpoint));
}

#[test]
fn test_xrpl_finality_anchor() {
    let checkpoint = FinalityCheckpoint {
        execution_root: "e".into(),
        receipt_root: "r".into(),
        snapshot_root: "s".into(),
        epoch_id: 3,
        verifier_quorum_proof: "proof".into(),
    };
    let anchor = create_settlement_anchor(&checkpoint, "xrpl-tx".into());
    assert!(anchor.confirmed);
    assert_eq!(anchor.xrpl_tx_hash, "xrpl-tx");
}

#[test]
fn test_epoch_aware_consensus() {
    let votes = vec![vote("v1", "r1", 1), vote("v2", "r1", 1), vote("v3", "r2", 2)];
    let epoch_1_votes: Vec<_> = votes.into_iter().filter(|v| v.epoch_id == 1).collect();
    let result = resolve_receipt_consensus(&epoch_1_votes).expect("consensus");
    assert_eq!(result.canonical_receipt_hash, "r1");
}
