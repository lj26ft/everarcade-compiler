//! Deterministic EverArcade developer platform signing API surface.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicRecord {
    pub id: &'static str,
    pub replay_safe: bool,
    pub authority_required: bool,
}

pub fn surface() -> DeterministicRecord {
    DeterministicRecord {
        id: "sdk/game-packager/src/signing",
        replay_safe: true,
        authority_required: true,
    }
}

pub fn validate_deterministic_surface() -> bool {
    let s = surface();
    s.replay_safe && s.authority_required && !s.id.is_empty()
}
