use execution_core::economics::{execution_cost::execution_cost, CostSchedule};

#[test]
fn deterministic_execution_cost() {
    let s = CostSchedule::default();
    assert_eq!(execution_cost(42, s), execution_cost(42, s));
}
