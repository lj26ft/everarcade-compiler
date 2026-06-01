use contract_api::protocol_records::{fields, ProtocolRecord, ReplayRecord};
use serde::{Deserialize, Serialize};

use super::receipt::ExecutionReceipt;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayLog {
    pub events: Vec<ProtocolRecord>,
}

impl ReplayLog {
    pub fn append_execution(&mut self, receipt: &ExecutionReceipt) {
        self.events.push(ProtocolRecord::Replay(ReplayRecord::new(
            "rustrig-executed",
            receipt.rustrig_id.clone(),
            fields(&[
                ("output_hash", receipt.output_hash.clone()),
                ("record_root", receipt.record_root.clone()),
                ("state_root", receipt.state_root.clone()),
            ]),
        )));
    }
}
