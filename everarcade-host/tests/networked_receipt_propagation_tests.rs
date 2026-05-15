use everarcade_host::distributed_receipts::{
    disk_store::DistributedReceiptDiskStore, execution_receipt::DistributedExecutionReceipt,
};
use everarcade_host::distributed_sync::{
    distributed_receipt_export::export_distributed_receipt,
    distributed_receipt_import::import_distributed_receipt,
};

#[test]
fn networked_receipt_propagation_converges() {
    let temp = std::env::temp_dir().join(format!(
        "everarcade_networked_receipt_{}_{}",
        std::process::id(),
        "node"
    ));
    let _ = std::fs::remove_dir_all(&temp);
    let store = DistributedReceiptDiskStore::new(&temp).unwrap();
    let receipt = DistributedExecutionReceipt {
        receipt_root: [1; 32],
        task_root: [2; 32],
        package_root: [3; 32],
        operator_id: [4; 32],
    };

    let exported = export_distributed_receipt(&receipt, [7; 32], [0; 32], [8; 32]).unwrap();
    import_distributed_receipt(&store, &exported).unwrap();

    let manifest = store.load_manifest().unwrap();
    assert_eq!(manifest.latest_replay_root, Some(exported.replay_root));

    let _ = std::fs::remove_dir_all(&temp);
}
