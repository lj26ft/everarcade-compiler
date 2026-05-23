use execution_core::federation::{
    checkpoint::FederationCheckpoint,
    recovery::{recover_continuity, FederationRecoveryState},
    replay_verify::ReplayVerificationInput,
    settlement::FederationSettlementJournal,
};
#[test]
fn recovery_equivalence() {
    let cp = FederationCheckpoint {
        federation_epoch: 1,
        previous_checkpoint_hash: "".into(),
        state_root: "s".into(),
        execution_journal_hash: "j".into(),
        receipt_root: "r".into(),
        replay_root: "rp".into(),
        settlement_root: "st".into(),
    };
    let st = FederationRecoveryState {
        checkpoints: vec![cp],
        settlement_journal: FederationSettlementJournal { entries: vec![] },
        restored_state_root: "s".into(),
    };
    let expected = ReplayVerificationInput {
        receipt_hash: "r".into(),
        state_root: "s".into(),
        replay_root: "rp".into(),
        settlement_root: "st".into(),
    };
    assert!(recover_continuity(&st, &expected).is_ok());
}
