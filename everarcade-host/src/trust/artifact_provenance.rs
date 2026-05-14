pub type Hash = [u8; 32];
use super::provenance::ProvenanceRecord;
pub fn artifact_provenance_valid(record: &ProvenanceRecord, artifact_root: Hash) -> bool {
    record.artifact_root == artifact_root
}
