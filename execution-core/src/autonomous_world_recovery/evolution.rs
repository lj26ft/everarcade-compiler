use super::runtime::AutonomousWorldRecoveryState;

pub fn evolve(state: &AutonomousWorldRecoveryState, input: &str) -> AutonomousWorldRecoveryState {
    let tick = state.tick + 1;
    let lineage = format!(
        "autonomous_world_recovery:{}:lineage:{tick}:{input}",
        state.id
    );
    let replay_tip = format!(
        "autonomous_world_recovery:{}:replay:{tick}:{input}",
        state.id
    );
    let continuity_root = format!(
        "autonomous_world_recovery:{}:continuity:{tick}:{lineage}:{replay_tip}",
        state.id
    );
    let mut append_only_history = state.append_only_history.clone();
    append_only_history.push(format!(
        "autonomous_world_recovery:{}:event:{tick}:{input}",
        state.id
    ));
    AutonomousWorldRecoveryState {
        id: state.id.clone(),
        tick,
        lineage,
        replay_tip,
        continuity_root,
        append_only_history,
    }
}
