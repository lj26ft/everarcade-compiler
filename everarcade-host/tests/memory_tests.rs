use everarcade_host::memory::{
    civilization_memory::CivilizationMemoryRecord, memory_root::memory_root,
    memory_validation::validate_memory_root,
};
#[test]
fn historical_replay_stability() {
    let rec = CivilizationMemoryRecord {
        civilization_root: [1; 32],
        replay_root: [2; 32],
        checkpoint_root: [3; 32],
        continuity_root: [4; 32],
        epoch_index: 7,
    };
    let a = memory_root(&[rec]);
    let b = memory_root(&[rec]);
    assert_eq!(a, b);
    assert!(validate_memory_root(&[rec], a));
}
