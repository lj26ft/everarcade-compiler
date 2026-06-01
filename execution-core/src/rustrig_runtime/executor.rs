use contract_api::protocol_records::{ProtocolRecord, RecordFields};
use contract_api::rustrig::{RustrigContext, RustrigOutput};
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::error::Result;
use super::registry::RustrigRegistry;

pub fn stable_hash<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("protocol records serialize deterministically");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

#[derive(Clone)]
pub struct RustrigExecutor {
    pub registry: RustrigRegistry,
}

impl RustrigExecutor {
    pub fn new(registry: RustrigRegistry) -> Self {
        Self { registry }
    }
    pub fn execute(
        &self,
        id: &str,
        ctx: &RustrigContext,
        payload: &RecordFields,
    ) -> Result<RustrigOutput> {
        self.registry.execute(id, ctx, payload)
    }
    pub fn record_root(records: &[ProtocolRecord]) -> String {
        stable_hash(&records)
    }
}
