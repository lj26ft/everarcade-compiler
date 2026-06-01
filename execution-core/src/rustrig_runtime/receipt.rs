use contract_api::protocol_records::ProtocolRecord;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub rustrig_id: String,
    pub version: String,
    pub input_hash: String,
    pub output_hash: String,
    pub record_count: usize,
    pub record_root: String,
    pub state_root: String,
    pub replay_root: String,
    pub checkpoint_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompositionReceipt {
    pub pipeline_id: String,
    pub receipts: Vec<ExecutionReceipt>,
    pub records: Vec<ProtocolRecord>,
    pub composition_root: String,
}
