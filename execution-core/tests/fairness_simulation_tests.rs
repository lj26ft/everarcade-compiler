use execution_core::simulation::fairness_simulation::fairness_index;

#[test]
fn fairness_simulation_is_stable() {
    assert_eq!(fairness_index(100, 4), 25);
    assert_eq!(fairness_index(100, 0), 0);
}
