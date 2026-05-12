use execution_core::simulation::interoperability_convergence::interoperability_converges;
#[test]
fn convergence_requires_same_root() { assert!(interoperability_converges([7;32],[7;32])); }
