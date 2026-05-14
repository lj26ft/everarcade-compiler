pub fn arbitration_replay_valid(dispute_root: [u8; 32], resolution_root: [u8; 32]) -> bool {
    dispute_root != [0; 32] && resolution_root != [0; 32]
}
