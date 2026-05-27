#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ObserverHydration {
    pub non_authoritative: bool,
    pub replay_derived: bool,
}

impl ObserverHydration {
    pub fn preserves_equivalence(&self) -> bool {
        self.non_authoritative && self.replay_derived
    }
}
