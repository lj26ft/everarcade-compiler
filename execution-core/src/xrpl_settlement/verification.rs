use super::{
    asset::verify_asset_lineage, continuity::SettlementContinuity, ledger::verify_ledger_checkpoint,
};

pub fn verify_settlement_continuity(continuity: &SettlementContinuity) -> bool {
    continuity
        .records
        .windows(2)
        .all(|w| w[0].checkpoint_lineage <= w[1].checkpoint_lineage)
}

pub fn verify_settlement_integrity(continuity: &SettlementContinuity) -> bool {
    verify_settlement_continuity(continuity)
        && continuity.records.iter().all(|record| {
            verify_asset_lineage(&record.asset_lineage)
                && verify_ledger_checkpoint(&super::ledger::LedgerCheckpoint {
                    ledger_index: record.checkpoint_lineage,
                    settlement_epoch: record.transaction_reference.settlement_epoch,
                    ledger_hash: record.settlement_proof.proof_root,
                })
        })
}

pub fn verify_asset_continuity(continuity: &SettlementContinuity) -> bool {
    continuity
        .records
        .iter()
        .all(|record| verify_asset_lineage(&record.asset_lineage))
}

pub fn inspect_settlement_lineage(continuity: &SettlementContinuity) -> usize {
    continuity.records.len()
}
pub fn inspect_asset_continuity(continuity: &SettlementContinuity) -> usize {
    continuity
        .records
        .iter()
        .map(|r| r.asset_lineage.ownership.len())
        .sum()
}
pub fn inspect_economic_replay(continuity: &SettlementContinuity) -> bool {
    verify_settlement_integrity(continuity)
}
