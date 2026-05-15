use everarcade_host::operator_recovery::{
    distributed_receipt_failover::distributed_receipt_failover_ready,
    receipt_reassignment_recovery::receipt_reassignment_recovery_ready,
    restart_receipt_resume::restart_receipt_resume_ready,
};
#[test]
fn failover_receipt_persistence_ready() {
    assert!(
        distributed_receipt_failover_ready()
            && receipt_reassignment_recovery_ready()
            && restart_receipt_resume_ready()
    );
}
