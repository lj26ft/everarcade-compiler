use execution_core::persistence::compression::ReplayCompressionManifest;
#[test]
fn compression_continuity() {
    let c = ReplayCompressionManifest {
        range_start: 0,
        range_end: 10,
        snapshot_root: "s".into(),
        continuity_anchor_root: "a".into(),
        compressed_chunks: 2,
    };
    assert!(c.restores_to("s", "a"));
    assert_eq!(c.canonical_hash().unwrap(), c.canonical_hash().unwrap());
}
