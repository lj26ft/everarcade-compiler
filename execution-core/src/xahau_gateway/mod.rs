use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XahauSettlementIntent {
    pub game_id: String,
    pub world_id: String,
    pub checkpoint_epoch: u64,
    pub checkpoint_root: String,
    pub settlement_root: String,
    pub asset_lineage_root: String,
    pub receipt_hash: String,
    pub runtime_commit_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XahauHookInvocation {
    pub hook_account: String,
    pub hook_name: String,
    pub invoke_transaction_type: String,
    pub parameters: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XahauProofAnchor {
    pub world_id: String,
    pub checkpoint_root: String,
    pub settlement_root: String,
    pub proof_hash: String,
    pub submitted_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XahauSettlementReceipt {
    pub world_id: String,
    pub checkpoint_epoch: u64,
    pub settlement_root: String,
    pub ledger_hash: String,
    pub ledger_index: u64,
    pub transaction_hash: String,
    pub accepted: bool,
    pub message: String,
}

pub fn settlement_proof_hash(intent: &XahauSettlementIntent) -> String {
    use sha2::{Digest, Sha256};

    let payload = format!(
        "{}:{}:{}:{}:{}:{}:{}:{}",
        intent.game_id,
        intent.world_id,
        intent.checkpoint_epoch,
        intent.checkpoint_root,
        intent.settlement_root,
        intent.asset_lineage_root,
        intent.receipt_hash,
        intent.runtime_commit_hash
    );

    hex::encode(Sha256::digest(payload.as_bytes()))
}
