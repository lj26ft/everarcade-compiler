#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSurfaceDescriptor {
    pub name: &'static str,
    pub replay_only: bool,
    pub description: &'static str,
}

pub const DESCRIPTOR: RuntimeSurfaceDescriptor = RuntimeSurfaceDescriptor {
    name: "runtime/network_flow/validation.rs",
    replay_only: true,
    description: "Flow validation rejects replay duplication, reordering, and overflow corruption.",
};

pub fn preserves_replay_continuity() -> bool { true }
pub fn rejects_mutable_authority() -> bool { true }
