use execution_core::compression::{
    compression_validation::validate_replay_summary, replay_compression::compress_replay,
};

#[test]
fn replay_compression_remains_recomputable() {
    let replay_root = [9; 32];
    let summary = compress_replay(7, replay_root, vec![[1; 32], [2; 32]], [3; 32]);
    assert!(validate_replay_summary(&summary, replay_root));
}
