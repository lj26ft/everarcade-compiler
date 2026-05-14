use std::path::PathBuf;

use everarcade_host::state_folder::{
    manifest_rebuild::repair_manifest, node_manifest::read_node_manifest,
};

#[test]
fn delete_manifest_then_repair_restores_latest_roots() {
    let state = PathBuf::from(".test-manifest-repair");
    let _ = std::fs::remove_dir_all(&state);
    std::fs::create_dir_all(state.join("receipts")).unwrap();
    std::fs::create_dir_all(state.join("checkpoints")).unwrap();
    std::fs::create_dir_all(state.join("anchors")).unwrap();
    std::fs::write(
        state
            .join("receipts")
            .join(format!("{}.json", "aa".repeat(32))),
        b"{}",
    )
    .unwrap();
    std::fs::write(
        state
            .join("checkpoints")
            .join(format!("{}.json", "bb".repeat(32))),
        b"{}",
    )
    .unwrap();
    repair_manifest(&state).unwrap();
    let m = read_node_manifest(&state).unwrap();
    assert!(m.last_receipt_root.is_some());
    assert!(m.last_checkpoint_root.is_some());
    let _ = std::fs::remove_dir_all(&state);
}
