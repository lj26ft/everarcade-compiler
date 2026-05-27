#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WindowSync {
    pub deterministic: bool,
}
