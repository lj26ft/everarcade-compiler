use everarcade_host::economic_runtime::{
    budget_execution::execute_budget_window, economic_replay_validation::validate_economic_replay,
};
#[test]
fn economic_continuity() {
    let root = execute_budget_window([9; 32], [9; 32]);
    assert!(validate_economic_replay(root, root).is_ok());
}
