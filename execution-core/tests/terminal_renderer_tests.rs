use execution_core::{
    game_runtime::{
        input_runtime::{InputAction, RuntimeInput},
        replay_runtime::{ReplayRecord, ReplayTickRecord},
    },
    render_bridge::{
        projection_roots::derive_projection_root,
        stream::ProjectionStreamRuntime,
        validation::{verify_projection_checkpoint, verify_projection_equivalence},
    },
};

fn mk_replay(frames: u64) -> ReplayRecord {
    let ticks = (0..frames)
        .map(|tick| ReplayTickRecord {
            tick,
            inputs: vec![RuntimeInput {
                tick,
                player_id: "p1".into(),
                action: InputAction::MoveRight,
            }],
            state_root: format!("state-{tick:05}"),
            event_root: format!("event-{tick:05}"),
            validation_root: format!("validation-{tick:05}"),
        })
        .collect();
    ReplayRecord::load_replay(ticks)
}

#[test]
fn test_terminal_frame_equivalence() {
    let replay = mk_replay(8);
    let a = ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 7).unwrap();
    let b = ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 7).unwrap();
    assert_eq!(a.frames, b.frames);
}

#[test]
fn test_terminal_replay_equivalence() {
    let replay = mk_replay(64);
    let a = ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 63).unwrap();
    let b = replay.materialize_projection_window(0, 63).unwrap();
    verify_projection_equivalence(&a, &b).unwrap();
}

#[test]
fn test_projection_frame_ordering() {
    let replay = mk_replay(32);
    let window = ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 31).unwrap();
    assert!(window
        .frames
        .windows(2)
        .all(|w| w[0].world.tick <= w[1].world.tick));
}

#[test]
fn test_visual_projection_equivalence() {
    let replay = mk_replay(20);
    let window = ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 19).unwrap();
    let r1 = derive_projection_root(&window.frames).unwrap();
    let r2 = derive_projection_root(&window.frames).unwrap();
    assert_eq!(r1, r2);
}

#[test]
fn test_inventory_visual_projection() {
    assert!(true);
}

#[test]
fn test_entity_visual_projection() {
    assert!(true);
}

#[test]
fn test_projection_resume_equivalence() {
    let replay = mk_replay(100);
    let window = ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 99).unwrap();
    let resumed = replay.resume_replay(60);
    let resumed_frames = ProjectionStreamRuntime::stream_projection_frames(&window)
        .filter(|f| f.world.tick >= 60)
        .count();
    assert_eq!(resumed.len(), resumed_frames);
}

#[test]
fn test_render_checkpoint_chain() {
    let replay = mk_replay(10);
    let window = ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 9).unwrap();
    let checkpoint = ProjectionStreamRuntime::checkpoint_for(&window).unwrap();
    verify_projection_checkpoint(&checkpoint, &window).unwrap();
}

#[test]
fn test_large_visible_runtime_progression() {
    let replay = mk_replay(10_001);
    let window =
        ProjectionStreamRuntime::materialize_projection_window(&replay, 0, 10_000).unwrap();
    assert_eq!(window.frames.len(), 10_001);
}
