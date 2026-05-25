use execution_core::simulation::{
    interaction::{
        propagate_interaction_event, resolve_multiplayer_interaction, verify_interaction_replay,
        InteractionEvent,
    },
    state::SimulationState,
};

#[test]
fn test_multiplayer_interaction_replay() {
    let mut events = vec![];
    let event = InteractionEvent {
        actor: "p1".into(),
        target: "p2".into(),
        kind: "combat".into(),
        amount: 2,
    };
    propagate_interaction_event(&mut events, event.clone());
    assert!(verify_interaction_replay(&events, &[event]));
}

#[test]
fn test_inventory_state_convergence() {
    let mut s1 = SimulationState::default();
    let mut s2 = SimulationState::default();
    let ev = InteractionEvent {
        actor: "p1".into(),
        target: "bag".into(),
        kind: "inventory".into(),
        amount: 5,
    };
    resolve_multiplayer_interaction(&mut s1, &ev);
    resolve_multiplayer_interaction(&mut s2, &ev);
    assert_eq!(s1.inventory, s2.inventory);
}
