use execution_core::federation::checkpoint::{validate_checkpoint_linkage, FederationCheckpoint};

#[test]
fn checkpoint_determinism() {
    let c = FederationCheckpoint {
        federation_epoch: 1,
        previous_checkpoint_hash: "".into(),
        state_root: "s".into(),
        execution_journal_hash: "j".into(),
        receipt_root: "r".into(),
        replay_root: "rp".into(),
        settlement_root: "st".into(),
    };
    assert_eq!(c.canonical_hash().unwrap(), c.canonical_hash().unwrap());
    assert!(validate_checkpoint_linkage(&[c]).is_ok());
}
