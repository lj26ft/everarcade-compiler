pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvenanceRecord {
    pub artifact_root: Hash,
    pub signer_root: Hash,
    pub lineage_root: Hash,
}
