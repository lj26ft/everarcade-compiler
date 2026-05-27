#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RecoveryTransport {
    pub deterministic: bool,
}
