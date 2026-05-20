use execution_core::xrpl_settlement::*;

#[test]
fn test_asset_ownership_transfer() {
    let mut lineage = AssetLineage::default();
    assign_asset_owner(&mut lineage, "sword-001", "player-001", 1).unwrap();
    transfer_asset_ownership(&mut lineage, "sword-001", "player-001", "player-002", 2).unwrap();
    assert_eq!(lineage.ownership.last().unwrap().owner_id, "player-002");
}

#[test]
fn test_asset_lineage_replay() {
    let mut lineage = AssetLineage::default();
    assign_asset_owner(&mut lineage, "asset-a", "owner-a", 1).unwrap();
    assert!(verify_asset_lineage(&lineage));
}

#[test]
fn test_settlement_record_determinism() {
    let mut lineage = AssetLineage::default();
    assign_asset_owner(&mut lineage, "asset-a", "owner-a", 1).unwrap();
    let tx = XRPLTransactionReference {
        transaction_hash: [7u8; 32],
        ledger_index: 10,
        settlement_epoch: 4,
        confirmation_lineage: 2,
    };
    let record = create_settlement_record(
        [9u8; 32],
        LedgerCheckpoint {
            ledger_index: 10,
            settlement_epoch: 4,
            ledger_hash: [4u8; 32],
        },
        tx.clone(),
        SettlementProof {
            proof_root: [1u8; 32],
            verifier_set_hash: [2u8; 32],
        },
        lineage.clone(),
    );
    let replayed = replay_settlement_lineage(&[record.clone()]);
    assert_eq!(replayed.records[0], record);
    assert!(verify_xrpl_reference(&tx).is_ok());
}

#[test]
fn test_economic_replay_convergence() {
    let mut lineage = AssetLineage::default();
    assign_asset_owner(&mut lineage, "asset-a", "owner-a", 1).unwrap();
    let tx = XRPLTransactionReference {
        transaction_hash: [7u8; 32],
        ledger_index: 10,
        settlement_epoch: 4,
        confirmation_lineage: 2,
    };
    let record = create_settlement_record(
        [9u8; 32],
        LedgerCheckpoint {
            ledger_index: 10,
            settlement_epoch: 4,
            ledger_hash: [4u8; 32],
        },
        tx,
        SettlementProof {
            proof_root: [1u8; 32],
            verifier_set_hash: [2u8; 32],
        },
        lineage,
    );
    let a = replay_settlement_lineage(&[record.clone()]);
    let b = replay_settlement_lineage(&[record]);
    assert!(verify_economic_replay(&a, &b));
}
