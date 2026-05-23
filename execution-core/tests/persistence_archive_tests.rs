use execution_core::persistence::archive::{validate_archive_lineage, WorldArchive};

#[test]
fn archive_determinism() {
    let a = WorldArchive {
        sequence: 1,
        previous_archive_hash: String::new(),
        world_state_root: "ws".into(),
        replay_root: "rp".into(),
        checkpoint_root: "cp".into(),
        economic_ledger_root: "el".into(),
        entity_lineage_root: "en".into(),
        federation_continuity_root: "fc".into(),
    };
    assert_eq!(a.canonical_hash().unwrap(), a.canonical_hash().unwrap());
    assert!(validate_archive_lineage(&[a]).is_ok());
}
