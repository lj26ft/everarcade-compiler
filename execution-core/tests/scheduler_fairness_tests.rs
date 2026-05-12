use execution_core::execution::fairness::fairness_score;

#[test]
fn fairness_is_deterministic() {
    assert_eq!(fairness_score(10, 4), fairness_score(10, 4));
}
