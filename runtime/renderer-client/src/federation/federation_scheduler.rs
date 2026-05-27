#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FederationScheduler {
    pub continuity_root_verified: bool,
    pub deterministic_windowing: bool,
}

impl FederationScheduler {
    pub fn operational(&self) -> bool {
        self.continuity_root_verified && self.deterministic_windowing
    }
}
