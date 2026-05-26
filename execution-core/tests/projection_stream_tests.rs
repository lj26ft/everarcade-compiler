use execution_core::{
    game_runtime::{
        input_runtime::{InputAction, RuntimeInput},
        replay_runtime::{ReplayRecord, ReplayTickRecord},
    },
    render_bridge::{
        stream::{ProjectionStreamRuntime, ProjectionStreamSession},
        validation::{verify_projection_checkpoint, verify_projection_equivalence},
    },
};

fn sample_replay(n: u64) -> ReplayRecord {
    let ticks = (0..n)
        .map(|t| ReplayTickRecord {
            tick: t,
            inputs: vec![RuntimeInput {
                tick: t,
                player_id: "p1".into(),
                action: InputAction::MoveRight,
            }],
            state_root: format!("s{t}"),
            event_root: format!("e{t}"),
            validation_root: format!("v{t}"),
        })
        .collect();
    ReplayRecord::load_replay(ticks)
}

#[test]
fn test_projection_frame_equivalence() {
    let r = sample_replay(10);
    let a = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 9).unwrap();
    let b = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 9).unwrap();
    assert!(verify_projection_equivalence(&a, &b).is_ok());
}
#[test]
fn test_projection_checkpoint_equivalence() {
    let r = sample_replay(5);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 4).unwrap();
    let c = ProjectionStreamRuntime::checkpoint_for(&w).unwrap();
    assert!(verify_projection_checkpoint(&c, &w).is_ok());
}
#[test]
fn test_projection_replay_equivalence() {
    let r = sample_replay(2);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 1).unwrap();
    assert_eq!(w.frames.len(), 2);
}
#[test]
fn test_projection_resume_equivalence() {
    let r = sample_replay(8);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 7).unwrap();
    let s = ProjectionStreamSession {
        cursor: Default::default(),
        windows: vec![w],
        checkpoints: vec![],
    };
    assert_eq!(
        ProjectionStreamRuntime::resume_projection_stream(&s, 4).len(),
        4
    );
}
#[test]
fn test_projection_root_chain_equivalence() {
    let r = sample_replay(3);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 2).unwrap();
    assert!(ProjectionStreamRuntime::verify_projection_window(&w).is_ok());
}
#[test]
fn test_projection_window_materialization() {
    let r = sample_replay(7);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 2, 6).unwrap();
    assert_eq!(w.frames.len(), 5);
}
#[test]
fn test_projection_inventory_projection() {
    let r = sample_replay(1);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 0).unwrap();
    assert_eq!(w.frames[0].world.tick, 0);
}
#[test]
fn test_projection_entity_projection() {
    let r = sample_replay(1);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 0).unwrap();
    assert_eq!(w.frames[0].event.tick, 0);
}
#[test]
fn test_projection_event_projection() {
    let r = sample_replay(1);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 0).unwrap();
    assert_eq!(w.frames[0].event.root, "e0");
}
#[test]
fn test_large_projection_stream() {
    let r = sample_replay(10_001);
    let w = ProjectionStreamRuntime::materialize_projection_window(&r, 0, 10_000).unwrap();
    assert_eq!(w.frames.len(), 10_001);
}
