use execution_core::simulation::sovereign_exchange::sovereign_exchange_lineage_valid;
#[test]
fn sovereign_exchange_tracks_lineage() {
    assert!(sovereign_exchange_lineage_valid([1; 32], [1; 32]));
}
