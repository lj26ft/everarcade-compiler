use super::runtime::CivilizationInteractionState;

pub fn evolve(state: &CivilizationInteractionState, input: &str) -> CivilizationInteractionState {
    let tick = state.tick + 1;
    let lineage = format!(
        "civilization_interaction:{}:lineage:{tick}:{input}",
        state.id
    );
    let replay_tip = format!(
        "civilization_interaction:{}:replay:{tick}:{input}",
        state.id
    );
    let continuity_root = format!(
        "civilization_interaction:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!(
        "civilization_interaction:{}:event:{tick}:{input}",
        state.id
    ));
    CivilizationInteractionState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
