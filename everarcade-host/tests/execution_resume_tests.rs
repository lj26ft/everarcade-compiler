use everarcade_host::distributed_receipts::receipt_store::DistributedExecutionReceipt;
use everarcade_host::operator_recovery::execution_resume::resume_execution;

#[test]
fn replay_continuity_preserved_on_resume() {
    let receipt = DistributedExecutionReceipt::new([1; 32], [2; 32], [7; 32]);
    assert!(resume_execution(&receipt, [7; 32]));
}
