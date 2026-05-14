pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionLineage {
    pub lineage_root: Hash,
    pub parent_lineage: Option<Hash>,
    pub assignment_id: Hash,
    pub checkpoint_root: Hash,
}
