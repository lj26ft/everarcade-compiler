use everarcade_sdk::*;

fn sample() -> ReplayTimeline {
    load_replay_timeline(
        vec![
            ReplayEvent {
                tick: 1,
                entity_id: "p1".into(),
                interaction: "combat".into(),
                partition: "a".into(),
                governance_event: None,
                economic_delta: -1,
            },
            ReplayEvent {
                tick: 2,
                entity_id: "p1".into(),
                interaction: "loot".into(),
                partition: "b".into(),
                governance_event: Some("vote".into()),
                economic_delta: 3,
            },
        ],
        vec!["g".into(), "r1".into(), "r2".into()],
    )
}

#[test]
fn test_replay_tick_stepping() {
    assert_eq!(step_replay_tick(&sample(), 1).len(), 1);
}
#[test]
fn test_replay_diff_detection() {
    let mut b = sample();
    b.world_state_roots[2] = "x".into();
    assert!(detect_simulation_divergence(&sample(), &b));
}
#[test]
fn test_replay_state_reconstruction() {
    assert_eq!(
        inspect_replay_state(&sample(), 2)
            .get("world_state_root")
            .unwrap(),
        "r2"
    );
}
#[test]
fn test_timeline_rendering() {
    assert!(inspect_replay_state(&sample(), 1).contains_key("tick"));
}
#[test]
fn test_entity_lineage_visualization() {
    assert_eq!(replay_entity_evolution(&sample(), "p1").len(), 2);
}
#[test]
fn test_interaction_trace_reconstruction() {
    assert_eq!(reconstruct_interaction_lineage(&sample(), "p1").len(), 2);
}
#[test]
fn test_wall_clock_rejection() {
    assert!(!verify_deterministic_contract(
        "std::time::SystemTime::now()"
    ));
}
#[test]
fn test_rng_violation_detection() {
    assert!(detect_nondeterministic_behavior("rand::thread_rng()").contains(&"unseeded randomness"));
}
#[test]
fn test_nondeterministic_iteration_detection() {
    assert!(detect_nondeterministic_behavior("HashMap::<u8,u8>::new()")
        .contains(&"nondeterministic iteration"));
}
