use execution_core::xrpl_settlement::*;

#[test]
fn test_federated_settlement_sync() {
    let mut lineage = AssetLineage::default();
    assign_asset_owner(&mut lineage, "sword-001", "player-001", 1).unwrap();
    let tx = XRPLTransactionReference {
        transaction_hash: [3u8; 32],
        ledger_index: 42,
        settlement_epoch: 7,
        confirmation_lineage: 1,
    };
    let record = create_settlement_record(
        [5u8; 32],
        LedgerCheckpoint {
            ledger_index: 42,
            settlement_epoch: 7,
            ledger_hash: [8u8; 32],
        },
        tx,
        SettlementProof {
            proof_root: [9u8; 32],
            verifier_set_hash: [10u8; 32],
        },
        lineage,
    );
    let mut local = SettlementContinuity::default();
    let incoming = SettlementContinuity {
        records: vec![record],
        continuity_root: [1u8; 32],
    };
    sync_settlement_continuity(&mut local, incoming);
    assert!(verify_federated_settlement(&local));
    assert!(verify_settlement_integrity(&local));
}
