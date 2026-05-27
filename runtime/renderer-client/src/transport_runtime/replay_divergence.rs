#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReplayDivergence {
    pub deterministic: bool,
}
