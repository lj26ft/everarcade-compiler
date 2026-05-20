use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct InteractionEvent {
    pub actor: String,
    pub target: String,
    pub kind: String,
    pub amount: u64,
}

pub fn propagate_interaction_event(events: &mut Vec<InteractionEvent>, event: InteractionEvent) {
    events.push(event);
    events.sort();
}

pub fn resolve_multiplayer_interaction(
    state: &mut crate::simulation::state::SimulationState,
    event: &InteractionEvent,
) {
    let key = format!("{}:{}", event.actor, event.target);
    *state.entities.entry(key).or_insert(0) += event.amount;
    if event.kind == "inventory" {
        *state.inventory.entry(event.actor.clone()).or_insert(0) += event.amount;
    }
    if event.kind == "economy" {
        state.economy_volume = state.economy_volume.saturating_add(event.amount);
    }
}

pub fn verify_interaction_replay(left: &[InteractionEvent], right: &[InteractionEvent]) -> bool {
    left == right
}
