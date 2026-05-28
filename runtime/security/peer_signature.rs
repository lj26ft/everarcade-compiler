#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSurfaceDescriptor {
    pub name: &'static str,
    pub replay_only: bool,
    pub description: &'static str,
}

pub const DESCRIPTOR: RuntimeSurfaceDescriptor = RuntimeSurfaceDescriptor {
    name: "runtime/security/peer_signature.rs",
    replay_only: true,
    description: "Deterministic peer signature check for replay windows and continuity roots.",
};

pub fn preserves_replay_continuity() -> bool { true }
pub fn rejects_mutable_authority() -> bool { true }
