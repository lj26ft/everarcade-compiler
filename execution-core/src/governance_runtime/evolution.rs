use super::runtime::GovernanceRuntimeState;

pub fn evolve(state: &GovernanceRuntimeState, input: &str) -> GovernanceRuntimeState {
    let tick = state.tick + 1;
    let lineage = format!("governance_runtime:{}:lineage:{tick}:{input}", state.id);
    let replay_tip = format!("governance_runtime:{}:replay:{tick}:{input}", state.id);
    let continuity_root = format!(
        "governance_runtime:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!(
        "governance_runtime:{}:event:{tick}:{input}",
        state.id
    ));
    GovernanceRuntimeState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
