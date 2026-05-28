#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FederationDeployment {
    pub replay_continuity_preserved: bool,
    pub reconstruction_only: bool,
}

impl FederationDeployment {
    pub fn activate() -> Self {
        Self {
            replay_continuity_preserved: true,
            reconstruction_only: true,
        }
    }

    pub fn validate(&self) -> bool {
        self.replay_continuity_preserved && self.reconstruction_only
    }
}
