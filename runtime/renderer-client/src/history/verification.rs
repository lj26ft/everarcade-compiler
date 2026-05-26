use super::{archive::CivilizationArchiveRestoration, provenance::ReplayProvenanceProof};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayEquivalenceProof {
    pub frame_equivalent: bool,
    pub archive_equivalent: bool,
    pub provenance_equivalent: bool,
    pub observer_equivalent: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalReplayEquivalenceResult {
    pub equivalent: bool,
}
#[derive(Debug, Default)]
pub struct HistoricalReplayEquivalenceRuntime;
impl HistoricalReplayEquivalenceRuntime {
    pub fn verify(
        archive: &CivilizationArchiveRestoration,
        proof: &ReplayProvenanceProof,
        expected_root: &str,
    ) -> HistoricalReplayEquivalenceResult {
        HistoricalReplayEquivalenceResult {
            equivalent: archive.frame_equivalent && proof.manifest.root.value == expected_root,
        }
    }
}
