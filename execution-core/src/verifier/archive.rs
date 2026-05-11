use crate::{ExecutionPlan, ExecutionReceipt};
use crate::state_engine::snapshot::StateSnapshot;

#[derive(Debug, Default)]
pub struct VerifierArchive {
    pub snapshots: Vec<StateSnapshot>,
    pub receipts: Vec<ExecutionReceipt>,
    pub dags: Vec<ExecutionPlan>,
}

impl VerifierArchive {
    pub fn append_snapshot(&mut self, snapshot: StateSnapshot) { self.snapshots.push(snapshot); }
    pub fn append_receipt(&mut self, receipt: ExecutionReceipt) { self.receipts.push(receipt); }
    pub fn append_dag(&mut self, dag: ExecutionPlan) { self.dags.push(dag); }
}
