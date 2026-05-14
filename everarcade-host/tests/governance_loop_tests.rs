use everarcade_host::governance_runtime::governance_loop::execute_governance_loop;
#[test] fn governance_replay_continuity() { let h=[1u8;32]; let c=execute_governance_loop(h,[2;32],[3;32],1,[4;32],[5;32]).unwrap(); assert_eq!(c.replay_root,[4;32]); }
