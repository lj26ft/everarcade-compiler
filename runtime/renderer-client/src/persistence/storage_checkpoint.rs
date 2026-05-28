#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReplayStorageCheckpointBoundary {
    pub replay_derived: bool,
    pub reconstruction_only: bool,
    pub corruption_rejected: bool,
}

impl ReplayStorageCheckpointBoundary {
    pub fn operational() -> Self {
        Self {
            replay_derived: true,
            reconstruction_only: true,
            corruption_rejected: true,
        }
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.replay_derived && self.reconstruction_only && self.corruption_rejected {
            Ok(())
        } else {
            Err("runtime continuity boundary rejected")
        }
    }
}
