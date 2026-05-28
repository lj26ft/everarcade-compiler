#![allow(dead_code)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SocietySyncContinuity {
    pub replay_lineage: &'static str,
    pub deterministic: bool,
    pub authority_writes_rejected: bool,
}

impl SocietySyncContinuity {
    pub fn active() -> Self {
        Self {
            replay_lineage: "preserved",
            deterministic: true,
            authority_writes_rejected: true,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.replay_lineage == "preserved" && self.deterministic && self.authority_writes_rejected {
            Ok(())
        } else {
            Err("civilization replay continuity divergence rejected")
        }
    }
}
