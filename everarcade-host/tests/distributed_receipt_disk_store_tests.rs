use everarcade_host::distributed_receipts::{
    disk_store::DistributedReceiptDiskStore, execution_receipt::DistributedExecutionReceipt,
};

#[test]
fn persisted_receipt_roundtrip_updates_manifest() {
    let temp = std::env::temp_dir().join(format!("everarcade_disk_store_{}", std::process::id()));
    let store = DistributedReceiptDiskStore::new(&temp).unwrap();
    let receipt = DistributedExecutionReceipt {
        receipt_root: [1; 32],
        task_root: [2; 32],
        package_root: [3; 32],
        operator_id: [4; 32],
    };
    store.persist_receipt([8; 32], [9; 32], &receipt).unwrap();
    let m = store.load_manifest().unwrap();
    assert_eq!(m.receipt_count, 1);
    assert_eq!(m.latest_receipt_root, Some([1; 32]));
}
