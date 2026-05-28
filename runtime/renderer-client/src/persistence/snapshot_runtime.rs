#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SnapshotRuntime;

impl SnapshotRuntime {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
