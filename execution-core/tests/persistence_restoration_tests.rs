use execution_core::persistence::{
    archive::WorldArchive, compression::ReplayCompressionManifest, restoration::restore_continuity,
    storage_lineage::StorageLineage,
};
#[test]
fn restoration_equivalence() {
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
    let c = ReplayCompressionManifest {
        range_start: 0,
        range_end: 10,
        snapshot_root: "s".into(),
        continuity_anchor_root: "a".into(),
        compressed_chunks: 2,
    };
    let l = StorageLineage {
        archive_lineage_root: "a".into(),
        checkpoint_lineage_root: "c".into(),
        replay_lineage_root: "r".into(),
        economic_lineage_root: "e".into(),
        migration_lineage_root: "m".into(),
    };
    let r1 = restore_continuity(&a, &c, &l).unwrap();
    let r2 = restore_continuity(&a, &c, &l).unwrap();
    assert_eq!(r1.continuity_root, r2.continuity_root);
}
