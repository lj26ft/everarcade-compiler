#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProtocolRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl ProtocolRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
