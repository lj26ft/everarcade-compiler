use super::epoch_simulation::validate_epoch_continuity;

pub fn run_long_horizon_convergence(epoch_indices: &[u64], node_roots: &[[u8; 32]]) -> bool {
    validate_epoch_continuity(epoch_indices) && node_roots.windows(2).all(|w| w[0] == w[1])
}
