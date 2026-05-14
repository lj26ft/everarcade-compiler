use execution_core::economics::ResourceUsage;
use execution_core::simulation::economic_simulation::simulate_usage;

#[test]
fn same_inputs_same_aggregate_usage() {
    let sample = ResourceUsage {
        execution_units: 1,
        replay_units: 2,
        proof_units: 3,
        storage_units: 4,
    };
    let a = simulate_usage(&[sample, sample]);
    let b = simulate_usage(&[sample, sample]);
    assert_eq!(a, b);
}
