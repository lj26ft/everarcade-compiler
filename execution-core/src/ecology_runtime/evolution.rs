use super::runtime::EcologyRuntimeState;

pub fn evolve(state: &EcologyRuntimeState, input: &str) -> EcologyRuntimeState {
    let tick = state.tick + 1;
    let lineage = format!("ecology_runtime:{}:lineage:{tick}:{input}", state.id);
    let replay_tip = format!("ecology_runtime:{}:replay:{tick}:{input}", state.id);
    let continuity_root = format!(
        "ecology_runtime:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!("ecology_runtime:{}:event:{tick}:{input}", state.id));
    EcologyRuntimeState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
