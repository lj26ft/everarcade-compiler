use std::path::PathBuf;

use everarcade_host::index::index_rebuild::rebuild_indexes;

#[test]
fn delete_indexes_then_rebuild_restores_index_files() {
    let state = PathBuf::from(".test-index-rebuild");
    let _ = std::fs::remove_dir_all(&state);
    for d in ["receipts", "checkpoints", "anchors", "manifests"] {
        std::fs::create_dir_all(state.join(d)).unwrap();
    }
    std::fs::write(state.join("receipts").join("a.json"), b"{}").unwrap();
    let report = rebuild_indexes(&state).unwrap();
    assert_eq!(report.rebuilt_receipts, 1);
    assert!(state.join("manifests/receipt.index").exists());
    let _ = std::fs::remove_dir_all(&state);
}
