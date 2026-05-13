use everarcade_host::{checkpoint_sync::checkpoint_convergence::checkpoint_converged,replay_sync::convergence::converged};
#[test] fn distributed_civilization_converges(){assert!(converged([3;32],[3;32]));assert!(checkpoint_converged([6;32],[6;32]));}
