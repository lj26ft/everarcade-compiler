use super::runtime::FactionRuntimeState;

pub fn validate(state: &FactionRuntimeState) -> Result<(), &'static str> {
    let expected = format!(
        "faction_runtime:{}:continuity:{}:{}:{}",
        state.id, state.tick, state.lineage, state.replay_tip
    );
    if state.continuity_root != expected {
        return Err("deterministic continuity divergence rejected");
    }
    if state.append_only_history.len() != state.tick as usize + 1 {
        return Err("hidden mutation rejected");
    }
    Ok(())
}

pub fn equivalent(a: &FactionRuntimeState, b: &FactionRuntimeState) -> Result<(), &'static str> {
    validate(a)?;
    validate(b)?;
    if a == b {
        Ok(())
    } else {
        Err("replay equivalence divergence rejected")
    }
}

pub fn reject_authority_mutation(replay_derived_write: bool) -> Result<(), &'static str> {
    if replay_derived_write {
        Err("replay-derived authority mutation rejected")
    } else {
        Ok(())
    }
}
