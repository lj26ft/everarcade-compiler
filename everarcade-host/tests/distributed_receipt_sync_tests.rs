use everarcade_host::distributed_receipts::receipt_store::DistributedExecutionReceipt;
use everarcade_host::distributed_sync::{
    receipt_export::serialize_receipts, receipt_import::import_receipts,
};

#[test]
fn synced_receipts_match() {
    let receipts = vec![DistributedExecutionReceipt::new(
        [1u8; 32], [2u8; 32], [3u8; 32],
    )];
    assert_eq!(receipts, import_receipts(serialize_receipts(&receipts)));
}
