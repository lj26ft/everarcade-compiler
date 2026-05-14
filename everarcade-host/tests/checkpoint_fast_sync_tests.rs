use everarcade_host::checkpoint_sync::{
    checkpoint_chain_validation::validate_checkpoint_chain, checkpoint_delta::CheckpointDelta,
};

#[test]
fn checkpoint_import_and_delta_validate() {
    let delta = CheckpointDelta {
        from_checkpoint_root: [1; 32],
        to_checkpoint_root: [2; 32],
        missing_receipts: 10,
    };
    assert!(validate_checkpoint_chain(&delta));
}
