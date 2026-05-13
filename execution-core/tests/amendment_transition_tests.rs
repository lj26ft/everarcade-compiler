use execution_core::amendment::{amendment_execution::execute_amendment, amendment_transition::transition_amendment};

#[test]
fn amendment_continuity() {
    let amendment = transition_amendment([1; 32], [2; 32]);
    assert_eq!(execute_amendment(&amendment), [2; 32]);
}
