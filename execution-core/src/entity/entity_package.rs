#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EntityPackage {
    pub execution_package: Vec<u8>,
    pub lineage: Vec<u8>,
    pub proofs: Vec<u8>,
    pub checkpoints: Vec<u8>,
    pub memory: Vec<u8>,
    pub settlement_history: Vec<u8>,
}
