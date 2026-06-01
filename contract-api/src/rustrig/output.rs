use serde::{Deserialize, Serialize};

use crate::protocol_records::ProtocolRecord;

/// Canonical output envelope accepted by the Rustrig runtime.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RustrigOutput {
    pub rustrig_id: String,
    pub version: String,
    pub records: Vec<ProtocolRecord>,
    pub output_hash: String,
}
