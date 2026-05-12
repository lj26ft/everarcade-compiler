use crate::federation::event::FederationEvent;
use crate::federation::reducer::{reduce_federation_transition, FederationState};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssemblySession {
    pub state: FederationState,
    pub session_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssemblyResult {
    pub session_id: String,
    pub final_state: FederationState,
    pub transition_hashes: Vec<String>,
}

pub fn reduce_assembly_session(session: AssemblySession, events: &[FederationEvent]) -> AssemblyResult {
    let mut state = session.state;
    let mut transition_hashes = Vec::new();
    for event in events {
        let result = reduce_federation_transition(state, event.clone());
        transition_hashes.push(result.transition_hash.clone());
        state = result.next;
    }
    AssemblyResult { session_id: session.session_id, final_state: state, transition_hashes }
}
