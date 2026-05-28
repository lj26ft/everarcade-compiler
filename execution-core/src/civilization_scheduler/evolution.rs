use super::runtime::CivilizationSchedulerState;

pub fn evolve(state: &CivilizationSchedulerState, input: &str) -> CivilizationSchedulerState {
    let tick = state.tick + 1;
    let lineage = format!("civilization_scheduler:{}:lineage:{tick}:{input}", state.id);
    let replay_tip = format!("civilization_scheduler:{}:replay:{tick}:{input}", state.id);
    let continuity_root = format!(
        "civilization_scheduler:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!(
        "civilization_scheduler:{}:event:{tick}:{input}",
        state.id
    ));
    CivilizationSchedulerState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
