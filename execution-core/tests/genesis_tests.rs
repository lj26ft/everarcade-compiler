use execution_core::genesis::{
    compute_genesis_root, create_genesis_checkpoint, initialize_world_timeline,
};

#[test]
fn test_genesis_root_is_deterministic() {
    assert_eq!(compute_genesis_root(), compute_genesis_root());
}

#[test]
fn test_genesis_checkpoint_is_deterministic() {
    let state = compute_genesis_root();
    assert_eq!(
        create_genesis_checkpoint(&state),
        create_genesis_checkpoint(&state)
    );
}

#[test]
fn test_genesis_timeline_is_deterministic() {
    assert_eq!(initialize_world_timeline(), initialize_world_timeline());
}
