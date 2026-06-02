use control_plane::provider::ProviderRecoveryReport;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRoots {
    pub continuity_root: String,
    pub replay_root: String,
    pub checkpoint_root: String,
}

impl RecoveryRoots {
    pub fn sample() -> Self {
        Self {
            continuity_root: "continuity-root".into(),
            replay_root: "replay-root".into(),
            checkpoint_root: "checkpoint-root".into(),
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.continuity_root.is_empty()
            || self.replay_root.is_empty()
            || self.checkpoint_root.is_empty()
        {
            Err("missing recovery root".into())
        } else {
            Ok(())
        }
    }
}

pub fn recover_runtime(
    runtime_id: &str,
    roots: RecoveryRoots,
) -> Result<ProviderRecoveryReport, String> {
    roots.validate()?;
    Ok(ProviderRecoveryReport {
        runtime_id: runtime_id.into(),
        checkpoint_root: roots.checkpoint_root,
        replay_root: roots.replay_root,
        continuity_root: roots.continuity_root,
        rejoined_federation: true,
    })
}
