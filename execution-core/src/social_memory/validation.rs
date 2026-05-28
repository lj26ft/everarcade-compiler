use super::runtime::SocialMemoryState;

pub fn validate(state: &SocialMemoryState) -> Result<(), &'static str> {
    let expected = format!(
        "social_memory:{}:continuity:{}:{}:{}",
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

pub fn equivalent(a: &SocialMemoryState, b: &SocialMemoryState) -> Result<(), &'static str> {
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
