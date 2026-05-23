use execution_core::federation::settlement::{FederationSettlementJournal, SettlementEntry};
#[test]
fn settlement_determinism() {
    let j = FederationSettlementJournal {
        entries: vec![SettlementEntry {
            execution_commitment: "e".into(),
            checkpoint_commitment: "c".into(),
            state_root_confirmation: "s".into(),
            replay_confirmation: "r".into(),
            continuity_ack: "a".into(),
        }],
    };
    assert_eq!(j.canonical_hash().unwrap(), j.canonical_hash().unwrap());
}
