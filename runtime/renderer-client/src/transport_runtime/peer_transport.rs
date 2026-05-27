#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PeerTransport {
    pub deterministic: bool,
}
