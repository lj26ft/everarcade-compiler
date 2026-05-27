#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CatchupScheduler {
    pub deterministic: bool,
}
