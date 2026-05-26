#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayContinuityProof { pub continuity_root: String, pub lineage_hash: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayArchiveProof { pub archive_id: String, pub continuity_root: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayBranchProof { pub branch_id: String, pub parent_root: String, pub branch_root: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProvenanceProofMaterialization { pub provenance_root: String, pub witness: String }
#[derive(Debug, Default)]
pub struct ReplayProofMaterializationRuntime;
impl ReplayProofMaterializationRuntime {
 pub fn continuity(root: &str) -> ReplayContinuityProof { ReplayContinuityProof { continuity_root: root.into(), lineage_hash: format!("lineage::{root}") } }
}
