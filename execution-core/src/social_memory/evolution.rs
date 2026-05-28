use super::runtime::SocialMemoryState;

pub fn evolve(state: &SocialMemoryState, input: &str) -> SocialMemoryState {
    let tick = state.tick + 1;
    let lineage = format!("social_memory:{}:lineage:{tick}:{input}", state.id);
    let replay_tip = format!("social_memory:{}:replay:{tick}:{input}", state.id);
    let continuity_root = format!(
        "social_memory:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!("social_memory:{}:event:{tick}:{input}", state.id));
    SocialMemoryState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
