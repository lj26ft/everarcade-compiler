use execution_core::xahau_gateway::{
    settlement_proof_hash, XahauHookInvocation, XahauProofAnchor, XahauSettlementIntent,
    XahauSettlementReceipt,
};

pub fn build_settlement_invocation(
    intent: &XahauSettlementIntent,
    hook_account: impl Into<String>,
) -> XahauHookInvocation {
    let proof_hash = settlement_proof_hash(intent);
    XahauHookInvocation {
        hook_account: hook_account.into(),
        hook_name: "settlement-proof-hook".to_string(),
        invoke_transaction_type: "Invoke".to_string(),
        parameters: vec![
            ("world_id".to_string(), intent.world_id.clone()),
            (
                "checkpoint_root".to_string(),
                intent.checkpoint_root.clone(),
            ),
            (
                "settlement_root".to_string(),
                intent.settlement_root.clone(),
            ),
            (
                "asset_lineage_root".to_string(),
                intent.asset_lineage_root.clone(),
            ),
            ("receipt_hash".to_string(), intent.receipt_hash.clone()),
            (
                "runtime_commit_hash".to_string(),
                intent.runtime_commit_hash.clone(),
            ),
            ("proof_hash".to_string(), proof_hash),
        ],
    }
}

pub fn anchor_from_intent(
    intent: &XahauSettlementIntent,
    now_unix_seconds: u64,
) -> XahauProofAnchor {
    XahauProofAnchor {
        world_id: intent.world_id.clone(),
        checkpoint_root: intent.checkpoint_root.clone(),
        settlement_root: intent.settlement_root.clone(),
        proof_hash: settlement_proof_hash(intent),
        submitted_at_unix_seconds: now_unix_seconds,
    }
}

pub fn settlement_receipt(
    intent: &XahauSettlementIntent,
    ledger_hash: impl Into<String>,
    ledger_index: u64,
    transaction_hash: impl Into<String>,
    accepted: bool,
    message: impl Into<String>,
) -> XahauSettlementReceipt {
    XahauSettlementReceipt {
        world_id: intent.world_id.clone(),
        checkpoint_epoch: intent.checkpoint_epoch,
        settlement_root: intent.settlement_root.clone(),
        ledger_hash: ledger_hash.into(),
        ledger_index,
        transaction_hash: transaction_hash.into(),
        accepted,
        message: message.into(),
    }
}
