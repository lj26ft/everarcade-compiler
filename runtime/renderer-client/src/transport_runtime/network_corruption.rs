#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NetworkCorruption {
    pub deterministic: bool,
}
