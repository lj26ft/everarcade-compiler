use execution_core::gameplay::{
    execution::AuthorityBoundary, replay::validation::validate_checkpoint, GameplayExecution,
    GameplayInput, GameplayRuntime, GameplayRuntimeError, GameplaySession,
};
use execution_core::multiplayer::{
    validation::reject_divergent_inputs, MultiplayerRuntime, PlayerInput,
};
use execution_core::persistence::runtime::validation::validate_checkpoint_restore;
use execution_core::scheduler::{
    recovery::recover_schedule, runtime::AuthoritativeScheduler, validation::validate_ordering,
};
use execution_core::session::validation::validate_session_equivalence;

fn runtime_with_player() -> GameplayRuntime {
    let mut session = GameplaySession::new("arena");
    session.attach_player("p1");
    GameplayRuntime::new(session)
}

fn input(frame: u64, delta: u64) -> GameplayInput {
    GameplayInput::new("p1", frame, delta, "auth:arena:p1")
}

#[test]
fn test_gameplay_tick_equivalence() {
    let mut left = runtime_with_player();
    let mut right = runtime_with_player();
    left.execute_tick(input(1, 7)).expect("left tick");
    right.execute_tick(input(1, 7)).expect("right tick");
    assert_eq!(left.world.state, right.world.state);
    assert_eq!(left.windows, right.windows);
}

#[test]
fn test_authoritative_scheduler_equivalence() {
    let mut left = runtime_with_player();
    let mut right = runtime_with_player();
    let mut sl = AuthoritativeScheduler::new(1);
    let mut sr = AuthoritativeScheduler::new(1);
    let e = GameplayExecution {
        boundary: AuthorityBoundary::DeterministicRuntime,
        scheduled_tick: 1,
        input: input(1, 3),
    };
    sl.execute(&mut left, e.clone()).expect("left scheduled");
    sr.execute(&mut right, e).expect("right scheduled");
    assert_eq!(sl.frames, sr.frames);
    assert_eq!(left.world.state, right.world.state);
    validate_ordering(&sl).expect("ordering");
}

#[test]
fn test_session_recovery_equivalence() {
    let mut runtime = runtime_with_player();
    runtime.execute_tick(input(1, 5)).expect("tick");
    let checkpoint = runtime.checkpoints.last().unwrap().clone();
    let restored = GameplayRuntime::restore(
        runtime.session.clone(),
        checkpoint,
        runtime.world.state.score,
    )
    .expect("restore");
    validate_session_equivalence(&runtime, &restored).expect("equivalent");
}

#[test]
fn test_multiplayer_input_equivalence() {
    let mut multi = MultiplayerRuntime::new("root:everarcade:gameplay:arena:v1");
    let ordered = multi
        .synchronize(vec![
            PlayerInput {
                player_id: "b".into(),
                frame: 1,
                delta: 1,
            },
            PlayerInput {
                player_id: "a".into(),
                frame: 1,
                delta: 2,
            },
        ])
        .expect("sync");
    assert_eq!(ordered[0].player_id, "a");
    reject_divergent_inputs(&ordered, &ordered).expect("same inputs");
}

#[test]
fn test_gameplay_replay_restoration() {
    let mut runtime = runtime_with_player();
    let window = runtime.execute_tick(input(1, 9)).expect("tick");
    let checkpoint = runtime.checkpoints.last().unwrap().clone();
    validate_checkpoint(&window, &checkpoint).expect("checkpoint matches window");
    let restored =
        GameplayRuntime::restore(runtime.session.clone(), checkpoint, 9).expect("restore");
    assert_eq!(restored.world.state, runtime.world.state);
}

#[test]
fn test_execution_checkpoint_restoration() {
    let mut runtime = runtime_with_player();
    runtime.execute_tick(input(1, 4)).expect("tick");
    let checkpoint = runtime.checkpoints.last().unwrap();
    validate_checkpoint_restore(checkpoint, &runtime.session.continuity_root)
        .expect("valid checkpoint");
}

#[test]
fn test_matchmaking_session_routing() {
    let mut registry = execution_core::session::runtime::SessionRuntime::default();
    let runtime = registry.create_session("arena");
    assert_eq!(registry.sessions.len(), 1);
    assert_eq!(
        runtime.session.continuity_root,
        "root:everarcade:gameplay:arena:v1"
    );
}

#[test]
fn test_observer_gameplay_hydration() {
    let mut runtime = runtime_with_player();
    runtime.execute_tick(input(1, 2)).expect("tick");
    let observer_state = runtime.windows.last().unwrap().state_root.clone();
    assert_eq!(observer_state, runtime.world.state.state_root);
    assert!(GameplayRuntime::reject_replay_derived_authority("observer-hydration").is_err());
}

#[test]
fn test_gameplay_divergence_rejection() {
    let mut runtime = runtime_with_player();
    assert_eq!(
        runtime.execute_tick(input(2, 1)),
        Err(GameplayRuntimeError::InvalidSchedule)
    );
}

#[test]
fn test_authority_mutation_rejection() {
    let mut runtime = runtime_with_player();
    let e = GameplayExecution {
        boundary: AuthorityBoundary::Renderer,
        scheduled_tick: 1,
        input: input(1, 1),
    };
    assert_eq!(
        runtime.execute_authoritative(e),
        Err(GameplayRuntimeError::UnauthorizedMutation)
    );
    assert_eq!(
        GameplayRuntime::reject_replay_derived_authority("replay-window"),
        Err(GameplayRuntimeError::UnauthorizedMutation)
    );
}

#[test]
fn test_runtime_execution_integrity() {
    let mut runtime = runtime_with_player();
    let mut scheduler = AuthoritativeScheduler::new(1);
    let e = GameplayExecution {
        boundary: AuthorityBoundary::DeterministicRuntime,
        scheduled_tick: 1,
        input: input(1, 6),
    };
    scheduler.execute(&mut runtime, e).expect("execute");
    let recovered = recover_schedule(scheduler.checkpoints.last().unwrap());
    assert_eq!(recovered.next_tick, 2);
    assert_eq!(runtime.continuity.latest_tick, 1);
}

#[test]
fn test_runtime_non_authoritative_renderer() {
    assert_eq!(
        GameplayRuntime::reject_replay_derived_authority("renderer"),
        Err(GameplayRuntimeError::UnauthorizedMutation)
    );
}
