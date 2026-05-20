use std::fs;

#[test]
fn test_empty_runtime_starts_successfully() {
    let dir = tempfile::tempdir().unwrap();
    let world = dir.path();
    fs::create_dir_all(world.join("journal")).unwrap();
    fs::create_dir_all(world.join("receipts")).unwrap();
    fs::create_dir_all(world.join("checkpoints")).unwrap();
    let root = everarcade_host::runtime_replay::replay_world(world).unwrap();
    let genesis = execution_core::genesis::compute_genesis_root();
    assert_eq!(root, genesis.world_root.world_root);
}
