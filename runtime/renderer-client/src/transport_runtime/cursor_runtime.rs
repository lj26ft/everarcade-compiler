#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CursorRuntimeRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl CursorRuntimeRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
