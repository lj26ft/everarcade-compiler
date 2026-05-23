use execution_core::persistence::storage_lineage::StorageLineage;
#[test]
fn storage_lineage_determinism() {
    let s = StorageLineage {
        archive_lineage_root: "a".into(),
        checkpoint_lineage_root: "c".into(),
        replay_lineage_root: "r".into(),
        economic_lineage_root: "e".into(),
        migration_lineage_root: "m".into(),
    };
    assert_eq!(s.canonical_hash().unwrap(), s.canonical_hash().unwrap());
}
