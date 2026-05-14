use everarcade_host::trust::{
    artifact_provenance::artifact_provenance_valid, provenance::ProvenanceRecord,
};
#[test]
fn provenance_stable() {
    let p = ProvenanceRecord {
        artifact_root: [5; 32],
        signer_root: [6; 32],
        lineage_root: [7; 32],
    };
    assert!(artifact_provenance_valid(&p, [5; 32]));
}
