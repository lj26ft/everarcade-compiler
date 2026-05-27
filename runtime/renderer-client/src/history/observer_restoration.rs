#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ObserverRestoration {
    pub non_authoritative: bool,
    pub replay_derived: bool,
}

impl ObserverRestoration {
    pub fn preserves_equivalence(&self) -> bool {
        self.non_authoritative && self.replay_derived
    }
}
