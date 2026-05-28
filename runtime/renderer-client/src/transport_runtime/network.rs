#![allow(dead_code)]

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NetworkRuntime {
    pub deterministic: bool,
    pub append_only: bool,
}

impl NetworkRuntime {
    pub fn validate(&self) -> bool {
        self.deterministic && self.append_only
    }
}
pub mod peer_handshake;
pub mod peer_identity;
pub mod peer_registry;
pub mod peer_runtime;
pub mod peer_sync;
pub mod peer_topology;
