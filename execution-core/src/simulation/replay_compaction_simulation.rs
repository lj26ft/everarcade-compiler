pub fn replay_equivalence(full_replay_root: [u8; 32], compressed_replay_root: [u8; 32]) -> bool {
    full_replay_root == compressed_replay_root
}
