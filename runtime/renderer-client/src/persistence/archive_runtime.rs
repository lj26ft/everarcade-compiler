#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArchiveRuntime;

impl ArchiveRuntime {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
