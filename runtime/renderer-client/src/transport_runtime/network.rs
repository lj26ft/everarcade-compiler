#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NetworkRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl NetworkRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
