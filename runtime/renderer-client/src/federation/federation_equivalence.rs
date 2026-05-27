#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FederationEquivalence {
    pub continuity_root_verified: bool,
    pub deterministic_windowing: bool,
}

impl FederationEquivalence {
    pub fn operational(&self) -> bool {
        self.continuity_root_verified && self.deterministic_windowing
    }
}
