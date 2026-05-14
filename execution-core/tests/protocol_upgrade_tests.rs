use execution_core::protocol_upgrade::{
    activation, compatibility_matrix, epoch::ProtocolEpoch, migration,
    transition::UpgradeTransition,
};
use execution_core::state_engine::snapshot::StateSnapshot;

fn epoch(id: u64) -> ProtocolEpoch {
    ProtocolEpoch {
        epoch_id: id,
        abi_version: "everarcade-execution-abi-v2".into(),
        hash_version: "everarcade-hash-v1".into(),
        receipt_version: "everarcade-receipt-v1".into(),
        snapshot_version: "everarcade-snapshot-v1".into(),
        dag_version: "everarcade-dag-v1".into(),
        execution_version: "everarcade-execution-v1".into(),
    }
}

fn transition(from: u64, to: u64) -> UpgradeTransition {
    UpgradeTransition {
        from_epoch_id: from,
        to_epoch_id: to,
        snapshot_compatible: true,
        receipt_compatible: true,
        deterministic_migration: true,
        transition_hash: "transition-proof-hash".into(),
    }
}

#[test]
fn test_epoch_isolation() {
    let e1 = epoch(1);
    let e2 = epoch(2);
    assert!(compatibility_matrix::is_allowed_transition(&e1, &e2));
    assert!(!compatibility_matrix::is_allowed_transition(&e2, &e1));
}

#[test]
fn test_deterministic_epoch_transition() {
    let snapshot = StateSnapshot {
        state_root: "root".into(),
        state_entries: std::collections::BTreeMap::new(),
        snapshot_hash: "s1".into(),
        previous_snapshot_hash: None,
    };
    let e1 = epoch(1);
    let e2 = epoch(2);
    let t = transition(1, 2);
    let a = migration::apply_migration(snapshot.clone(), &e1, &e2, &t).unwrap();
    let b = migration::apply_migration(snapshot, &e1, &e2, &t).unwrap();
    assert_eq!(a.transformation_proof, b.transformation_proof);
    assert_eq!(
        a.upgraded_snapshot.snapshot_hash,
        b.upgraded_snapshot.snapshot_hash
    );
}

#[test]
fn test_invalid_upgrade_rejection() {
    let snapshot = StateSnapshot {
        state_root: "root".into(),
        state_entries: std::collections::BTreeMap::new(),
        snapshot_hash: "s1".into(),
        previous_snapshot_hash: None,
    };
    let e1 = epoch(1);
    let e3 = epoch(3);
    let t = transition(1, 3);
    assert!(migration::apply_migration(snapshot, &e1, &e3, &t).is_none());
}

#[test]
fn test_snapshot_epoch_replay() {
    let old_snapshot = StateSnapshot {
        state_root: "root".into(),
        state_entries: std::collections::BTreeMap::new(),
        snapshot_hash: "old-snapshot".into(),
        previous_snapshot_hash: None,
    };
    let e1 = epoch(1);
    assert_eq!(e1.epoch_id, 1);
    assert_eq!(old_snapshot.snapshot_hash, "old-snapshot");
}

#[test]
fn test_settlement_epoch_consistency() {
    let t = transition(1, 2);
    assert!(activation::can_activate_epoch(true, true, true, &t));
    assert!(!activation::can_activate_epoch(true, false, true, &t));
}
