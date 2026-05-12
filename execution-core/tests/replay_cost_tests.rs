use execution_core::economics::{replay_cost::replay_cost, CostSchedule};

#[test]
fn deterministic_replay_cost() {
    let s = CostSchedule::default();
    assert_eq!(replay_cost(5, s), replay_cost(5, s));
}
