use super::{decision::AiDecision, runtime::AiRuntime};

pub fn decisions_are_deterministic(decisions: &[AiDecision]) -> bool {
    decisions
        .windows(2)
        .all(|w| w[0].entity_id <= w[1].entity_id)
        && decisions.iter().all(|d| !d.replay_root.is_empty())
}
pub fn ai_equivalent(a: &AiRuntime, b: &AiRuntime) -> bool {
    a.decisions == b.decisions && a.memory == b.memory
}
