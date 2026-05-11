use crate::{ExecutionPlan, ExecutionReceipt};
use crate::state_engine::snapshot::StateSnapshot;

#[derive(Debug, Clone)]
pub enum SyncObject {
    Snapshot(StateSnapshot),
    Receipt(ExecutionReceipt),
    Dag(ExecutionPlan),
    Proof(Vec<u8>),
}

#[derive(Debug, Default)]
pub struct VerifierSync {
    pub snapshots: Vec<StateSnapshot>,
    pub receipts: Vec<ExecutionReceipt>,
    pub dags: Vec<ExecutionPlan>,
    pub proofs: Vec<Vec<u8>>,
}

impl VerifierSync {
    pub fn ingest(&mut self, object: SyncObject) {
        match object {
            SyncObject::Snapshot(snapshot) => self.snapshots.push(snapshot),
            SyncObject::Receipt(receipt) => self.receipts.push(receipt),
            SyncObject::Dag(dag) => self.dags.push(dag),
            SyncObject::Proof(proof) => self.proofs.push(proof),
        }
    }
}
