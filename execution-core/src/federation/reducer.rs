use crate::federation::event::FederationEvent;
use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationState {
    pub federation_id: String,
    pub members: Vec<String>,
    pub governance_root: String,
    pub treaty_root: String,
    pub constitutional_root: String,
    pub replay_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationTransitionResult {
    pub next: FederationState,
    pub transition_hash: String,
}

pub fn reduce_federation_transition(previous: FederationState, event: FederationEvent) -> FederationTransitionResult {
    let mut next = previous.clone();
    match &event {
        FederationEvent::MemberJoined { member_id } => next.members.push(member_id.clone()),
        FederationEvent::MemberExited { member_id } => next.members.retain(|m| m != member_id),
        FederationEvent::TreatyEstablished { treaty_id, .. } => next.treaty_root = treaty_id.clone(),
        FederationEvent::TreatySuperseded { new_treaty_id, .. } => next.treaty_root = new_treaty_id.clone(),
        FederationEvent::ConstitutionAmended { constitutional_root, .. } => next.constitutional_root = constitutional_root.clone(),
        FederationEvent::ResolutionFinalized { resolution_id, .. } => next.governance_root = resolution_id.clone(),
        FederationEvent::FederationMigrated { continuity_root, .. } => next.replay_root = continuity_root.clone(),
        _ => {}
    }

    next.members.sort();
    next.members.dedup();

    let event_hash = hash_bytes(format!("{:?}", event).as_bytes());
    let transition_hash = hash_bytes(
        format!(
            "{}|{}|{}|{}|{}|{}|{}",
            previous.federation_id,
            event_hash,
            next.members.join(","),
            next.governance_root,
            next.treaty_root,
            next.constitutional_root,
            next.replay_root
        )
        .as_bytes(),
    );
    next.replay_root = transition_hash.clone();

    FederationTransitionResult { next, transition_hash }
}
