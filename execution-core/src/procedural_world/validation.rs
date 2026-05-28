use super::runtime::ProceduralWorldState;

pub fn validate(state: &ProceduralWorldState) -> Result<(), &'static str> {
    let expected = format!(
        "procedural_world:{}:continuity:{}:{}:{}",
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

pub fn equivalent(a: &ProceduralWorldState, b: &ProceduralWorldState) -> Result<(), &'static str> {
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
