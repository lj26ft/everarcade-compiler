use everarcade_host::checkpoint_sync::{
    checkpoint_export::{export_checkpoint_bytes, TransferCheckpoint},
    checkpoint_import::import_checkpoint_bytes,
};

#[test]
fn same_transferred_checkpoint_same_root() {
    let checkpoint = TransferCheckpoint {
        checkpoint_root: [7; 32],
        state_bytes: vec![1, 2, 3],
    };
    let bytes = export_checkpoint_bytes(&checkpoint);
    let decoded = import_checkpoint_bytes(&bytes).unwrap();
    assert_eq!(checkpoint.checkpoint_root, decoded.checkpoint_root);
}
