use everarcade_host::governance_sync::governance_convergence::convergence_report;
#[test]
fn governance_converges() {
    assert!(convergence_report([1; 32], [1; 32]));
}
