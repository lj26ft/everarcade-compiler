use super::runtime::SocietyRuntimeState;

pub fn evolve(state: &SocietyRuntimeState, input: &str) -> SocietyRuntimeState {
    let tick = state.tick + 1;
    let lineage = format!("society_runtime:{}:lineage:{tick}:{input}", state.id);
    let replay_tip = format!("society_runtime:{}:replay:{tick}:{input}", state.id);
    let continuity_root = format!(
        "society_runtime:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!("society_runtime:{}:event:{tick}:{input}", state.id));
    SocietyRuntimeState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
