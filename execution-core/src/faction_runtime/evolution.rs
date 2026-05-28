use super::runtime::FactionRuntimeState;

pub fn evolve(state: &FactionRuntimeState, input: &str) -> FactionRuntimeState {
    let tick = state.tick + 1;
    let lineage = format!("faction_runtime:{}:lineage:{tick}:{input}", state.id);
    let replay_tip = format!("faction_runtime:{}:replay:{tick}:{input}", state.id);
    let continuity_root = format!(
        "faction_runtime:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!("faction_runtime:{}:event:{tick}:{input}", state.id));
    FactionRuntimeState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
