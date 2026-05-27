#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeerRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl PeerRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
