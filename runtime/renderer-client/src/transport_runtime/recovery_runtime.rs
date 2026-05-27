#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RecoveryRuntimeRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl RecoveryRuntimeRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
