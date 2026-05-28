#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Chunk;

impl Chunk {
    pub fn is_non_authoritative(&self) -> bool {
        true
    }
}
