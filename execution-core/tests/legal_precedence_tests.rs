use execution_core::simulation::legal_precedence::simulate_legal_precedence;

#[test]
fn legal_precedence_validation() {
    assert!(simulate_legal_precedence(3));
}
