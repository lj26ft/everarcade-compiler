use serde::{Deserialize, Serialize};

use super::{hash_hex, RuntimeNodeLifecycle};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostRuntimeStatus {
    pub lifecycle: RuntimeNodeLifecycle,
    pub continuity_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostOperationReceipt {
    pub operation: String,
    pub operation_hash: String,
    pub continuity_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeHostSession {
    pub operations: Vec<HostOperationReceipt>,
}

pub struct RuntimeHost {
    pub status: HostRuntimeStatus,
    pub session: RuntimeHostSession,
}
impl RuntimeHost {
    pub fn new() -> Self {
        Self {
            status: HostRuntimeStatus {
                lifecycle: RuntimeNodeLifecycle::Genesis,
                continuity_root: hash_hex("genesis"),
            },
            session: RuntimeHostSession { operations: vec![] },
        }
    }
    pub fn apply(
        &mut self,
        operation: &str,
        lifecycle: RuntimeNodeLifecycle,
    ) -> HostOperationReceipt {
        self.status.lifecycle = lifecycle;
        self.status.continuity_root =
            hash_hex(format!("{}:{}", self.status.continuity_root, operation));
        let rec = HostOperationReceipt {
            operation: operation.to_string(),
            operation_hash: hash_hex(operation),
            continuity_root: self.status.continuity_root.clone(),
        };
        self.session.operations.push(rec.clone());
        rec
    }
}
