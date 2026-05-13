use execution_core::simulation::constitutional_continuity::validate_constitutional_continuity;

#[test]
fn constitutional_continuity() {
    assert!(validate_constitutional_continuity([1;32], [2;32], [3;32]));
}
