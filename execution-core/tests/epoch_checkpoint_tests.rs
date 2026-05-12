use execution_core::epoch::epoch_checkpoint::EpochCheckpoint;

#[test]
fn epoch_checkpoint_lineage_exists() {
    let checkpoint = EpochCheckpoint { epoch_index: 4, checkpoint_root: [7;32], parent_checkpoint: Some([6;32]) };
    assert_eq!(checkpoint.epoch_index, 4);
}
