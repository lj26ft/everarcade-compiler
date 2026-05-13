use everarcade_host::convergence::convergence_engine::evaluate;
#[test] fn convergence_determinism(){assert!(evaluate([7;32],[7;32]).converged);assert!(!evaluate([7;32],[8;32]).converged);}
