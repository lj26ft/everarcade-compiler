use execution_core::sync::{replay_window::{ReplayWindow, validate_replay_window}, sync_window::validate_sync_window};
#[test] fn window_validates(){ let w=ReplayWindow{start_index:0,end_index:0,receipts:vec![]}; assert!(validate_replay_window(&w)); assert!(validate_sync_window(&w,true,true)); }
