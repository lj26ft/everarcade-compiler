use everarcade_host::distributed_receipts::execution_receipt::DistributedExecutionReceipt;
use everarcade_host::distributed_sync::{
    distributed_receipt_export::export_distributed_receipt,
    distributed_receipt_validation::validate_receipt_package,
};

#[test]
fn package_exports_and_validates() {
    let receipt = DistributedExecutionReceipt {
        receipt_root: [1; 32],
        task_root: [2; 32],
        package_root: [3; 32],
        operator_id: [4; 32],
    };
    let pkg = export_distributed_receipt(&receipt, [7; 32], [8; 32], [9; 32]).unwrap();
    assert_eq!(validate_receipt_package(&pkg).unwrap(), receipt);
}
