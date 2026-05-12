use execution_core::economics::{proof_cost::proof_cost, CostSchedule};

#[test]
fn deterministic_proof_cost() {
    let s = CostSchedule::default();
    assert_eq!(proof_cost(9, s), proof_cost(9, s));
}
