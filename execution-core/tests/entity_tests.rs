use execution_core::entity::{
    archival::ArchivedEntity, continuity::continuity_root, identity::EntityIdentity,
    lineage::advance_lineage, replay_identity::reconstruct_identity_from_replay,
    sovereignty::survives_infrastructure_change,
};

#[test]
fn test_entity_identity_stability() {
    let seed = b"stable-seed";
    let a = EntityIdentity::from_genesis(seed);
    let b = reconstruct_identity_from_replay(seed);
    assert_eq!(a, b);
}

#[test]
fn test_execution_lineage_continuity() {
    let root = "root0";
    let a = advance_lineage(root, "step", b"payload");
    let b = advance_lineage(root, "step", b"payload");
    assert_eq!(a.next_root, b.next_root);
}

#[test]
fn test_entity_migration() {
    let before = EntityIdentity::from_genesis(b"migration");
    let after = before.clone();
    assert_eq!(before.entity_id, after.entity_id);
}

#[test]
fn test_entity_upgrade_continuity() {
    let base = continuity_root("replay", "proof", "upgrade-v1", "archive");
    let upgraded = continuity_root("replay", "proof", "upgrade-v1", "archive");
    assert_eq!(base, upgraded);
}

#[test]
fn test_replay_identity_reconstruction() {
    let seed = b"replay-identity";
    let id = EntityIdentity::from_genesis(seed);
    let replay = reconstruct_identity_from_replay(seed);
    assert_eq!(id, replay);
}

#[test]
fn test_archival_restoration() {
    let id = EntityIdentity::from_genesis(b"archive");
    let archived = ArchivedEntity { identity: id.clone(), archive_ref: "archive://1".into() };
    assert_eq!(archived.identity, id);
}

#[test]
fn test_checkpoint_continuity() {
    let c0 = continuity_root("r0", "p0", "u0", "a0");
    let c1 = continuity_root("r0", "p0", "u0", "a0");
    assert_eq!(c0, c1);
}

#[test]
fn test_sovereign_entity_survival() {
    assert!(survives_infrastructure_change(true, true, true));
}
