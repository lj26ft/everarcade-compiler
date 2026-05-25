use execution_core::simulation::{
    checkpoint::SimulationCheckpoint,
    engine::advance_simulation_tick,
    interaction::InteractionEvent,
    replay::replay_simulation_timeline,
    scheduler::{schedule_simulation_execution, verify_execution_schedule},
    state::SimulationState,
};

#[test]
fn test_simulation_tick_determinism() {
    let mut s1 = SimulationState::default();
    let mut s2 = SimulationState::default();
    let events = vec![];
    let t1 = advance_simulation_tick(&mut s1, 1, &events).unwrap();
    let t2 = advance_simulation_tick(&mut s2, 1, &events).unwrap();
    assert_eq!(t1.simulation_hash, t2.simulation_hash);
}

#[test]
fn test_world_state_replay() {
    let mut state = SimulationState::default();
    let tick = advance_simulation_tick(&mut state, 1, &[]).unwrap();
    let cp = SimulationCheckpoint {
        tick,
        state: state.clone(),
    };
    let replay = replay_simulation_timeline(&[cp]).unwrap();
    assert_eq!(state, replay);
}

#[test]
fn test_scheduler_execution_order() {
    let mut events = vec![
        InteractionEvent {
            actor: "b".into(),
            target: "a".into(),
            kind: "economy".into(),
            amount: 1,
        },
        InteractionEvent {
            actor: "a".into(),
            target: "b".into(),
            kind: "economy".into(),
            amount: 1,
        },
    ];
    schedule_simulation_execution(&mut events);
    assert!(verify_execution_schedule(&events));
}
