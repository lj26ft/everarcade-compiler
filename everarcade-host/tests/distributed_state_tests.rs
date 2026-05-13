use everarcade_host::convergence::distributed_state::DistributedState;
#[test] fn distributed_state_holds_roots(){let s=DistributedState{replay_root:[1;32],checkpoint_root:[2;32]};assert_ne!(s.replay_root,s.checkpoint_root);}
