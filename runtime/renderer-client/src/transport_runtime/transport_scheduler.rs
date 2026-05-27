#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TransportSchedulerRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl TransportSchedulerRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
