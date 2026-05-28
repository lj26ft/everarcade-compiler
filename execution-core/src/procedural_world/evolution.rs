use super::runtime::ProceduralWorldState;

pub fn evolve(state: &ProceduralWorldState, input: &str) -> ProceduralWorldState {
    let tick = state.tick + 1;
    let lineage = format!("procedural_world:{}:lineage:{tick}:{input}", state.id);
    let replay_tip = format!("procedural_world:{}:replay:{tick}:{input}", state.id);
    let continuity_root = format!(
        "procedural_world:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!(
        "procedural_world:{}:event:{tick}:{input}",
        state.id
    ));
    ProceduralWorldState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
