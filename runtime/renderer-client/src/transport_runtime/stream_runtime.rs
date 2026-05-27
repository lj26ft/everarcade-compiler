#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StreamRuntimeRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl StreamRuntimeRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
