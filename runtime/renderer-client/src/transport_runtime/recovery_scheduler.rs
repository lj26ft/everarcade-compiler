#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RecoveryScheduler {
    pub deterministic: bool,
}
