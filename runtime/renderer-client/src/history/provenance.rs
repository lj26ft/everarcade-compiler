#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProvenanceRoot {
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProvenanceManifest {
    pub timeline_id: String,
    pub root: ReplayProvenanceRoot,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProvenanceProof {
    pub manifest: ReplayProvenanceManifest,
    pub witness: String,
}
impl ReplayProvenanceProof {
    pub fn verify(&self, expected: &ReplayProvenanceRoot) -> bool {
        self.manifest.root == *expected
    }
}
