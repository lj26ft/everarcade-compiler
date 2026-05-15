use everarcade_host::distributed_receipts::{
    disk_store::DistributedReceiptDiskStore, execution_receipt::DistributedExecutionReceipt,
};
use everarcade_host::recovery_scan::distributed_receipt_recovery::recover_distributed_receipt_manifest;
#[test]
fn manifest_rebuild_preserves_latest_root() {
    let temp = std::env::temp_dir().join(format!("everarcade_rebuild_{}", std::process::id()));
    let store = DistributedReceiptDiskStore::new(&temp).unwrap();
    let receipt = DistributedExecutionReceipt {
        receipt_root: [9; 32],
        task_root: [2; 32],
        package_root: [3; 32],
        operator_id: [4; 32],
    };
    store.persist_receipt([8; 32], [7; 32], &receipt).unwrap();
    std::fs::remove_file(temp.join(".everarcade/distributed_receipts/manifest.json")).unwrap();
    let rebuilt = recover_distributed_receipt_manifest(&temp).unwrap();
    assert_eq!(rebuilt.latest_receipt_root, Some([9; 32]));
    assert_eq!(rebuilt.latest_replay_root, Some([8; 32]));
    assert_eq!(rebuilt.latest_checkpoint_root, Some([7; 32]));
}
