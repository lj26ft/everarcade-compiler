use execution_core::simulation::long_horizon_simulation::run_long_horizon_convergence;

#[test]
fn long_horizon_convergence_remains_deterministic() {
    assert!(run_long_horizon_convergence(
        &[0, 1, 2, 3],
        &[[9; 32], [9; 32], [9; 32], [9; 32]]
    ));
}
