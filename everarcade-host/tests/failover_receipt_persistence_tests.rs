use everarcade_host::distributed_receipts::{
    disk_store::DistributedReceiptDiskStore, execution_receipt::DistributedExecutionReceipt,
};

#[test]
fn failover_receipt_persistence_ready() {
    let temp = std::env::temp_dir().join(format!("everarcade_failover_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&temp);

    let store = DistributedReceiptDiskStore::new(&temp).unwrap();
    store
        .persist_receipt(
            [8; 32],
            [7; 32],
            &DistributedExecutionReceipt {
                receipt_root: [9; 32],
                task_root: [2; 32],
                package_root: [3; 32],
                operator_id: [4; 32],
            },
        )
        .unwrap();

    let restarted = DistributedReceiptDiskStore::new(&temp).unwrap();
    let manifest = restarted.load_manifest().unwrap();
    assert_eq!(manifest.latest_receipt_root, Some([9; 32]));
    assert_eq!(manifest.latest_replay_root, Some([8; 32]));
    assert_eq!(manifest.latest_checkpoint_root, Some([7; 32]));

    let _ = std::fs::remove_dir_all(&temp);
}
