use execution_core::simulation::economic_convergence::economic_converges;
#[test] fn sovereign_economic_convergence(){ let r=[7u8;32]; assert!(economic_converges(r,r)); }
