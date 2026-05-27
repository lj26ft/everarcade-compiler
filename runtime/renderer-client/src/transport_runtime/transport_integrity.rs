#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TransportIntegrity {
    pub deterministic: bool,
}
