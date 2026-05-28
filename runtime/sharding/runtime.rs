#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSurfaceDescriptor {
    pub name: &'static str,
    pub replay_only: bool,
    pub description: &'static str,
}

pub const DESCRIPTOR: RuntimeSurfaceDescriptor = RuntimeSurfaceDescriptor {
    name: "runtime/sharding/runtime.rs",
    replay_only: true,
    description: "Shard runtime partitions replay windows while preserving replay equivalence.",
};

pub fn preserves_replay_continuity() -> bool { true }
pub fn rejects_mutable_authority() -> bool { true }
