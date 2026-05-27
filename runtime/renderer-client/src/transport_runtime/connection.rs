#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConnectionRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl ConnectionRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
