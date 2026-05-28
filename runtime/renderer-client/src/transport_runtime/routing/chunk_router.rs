#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChunkRouter;

impl ChunkRouter {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
