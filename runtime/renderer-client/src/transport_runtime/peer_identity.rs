#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeerIdentity {
    pub deterministic: bool,
}
