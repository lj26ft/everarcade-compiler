#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayVerificationInput {
    pub receipt_hash: String,
    pub state_root: String,
    pub replay_root: String,
    pub settlement_root: String,
}

pub fn verify_replay_equivalence(
    local: &ReplayVerificationInput,
    remote: &ReplayVerificationInput,
) -> Result<(), String> {
    if local.receipt_hash != remote.receipt_hash {
        return Err("receipt hash mismatch".into());
    }
    if local.state_root != remote.state_root {
        return Err("state root mismatch".into());
    }
    if local.replay_root != remote.replay_root {
        return Err("replay root mismatch".into());
    }
    if local.settlement_root != remote.settlement_root {
        return Err("settlement root mismatch".into());
    }
    Ok(())
}
