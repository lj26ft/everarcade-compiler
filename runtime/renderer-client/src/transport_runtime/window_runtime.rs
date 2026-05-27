#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WindowRuntimeRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl WindowRuntimeRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
