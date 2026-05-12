use execution_core::simulation::sovereign_simulation::simulate_sovereign_isolation;

#[test]
fn sovereign_simulation_reports_determinism() {
    let result = simulate_sovereign_isolation(3);
    assert_eq!(result.isolated_domains, 3);
    assert!(result.deterministic);
}
