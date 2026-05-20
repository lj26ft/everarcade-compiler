use super::{continuity::SettlementContinuity, settlement::SettlementRecord};

pub fn replay_settlement_lineage(records: &[SettlementRecord]) -> SettlementContinuity {
    let mut replayed = records.to_vec();
    replayed.sort_by(|a, b| {
        a.transaction_reference
            .settlement_epoch
            .cmp(&b.transaction_reference.settlement_epoch)
            .then(a.checkpoint_lineage.cmp(&b.checkpoint_lineage))
    });
    SettlementContinuity {
        records: replayed,
        continuity_root: [1u8; 32],
    }
}

pub fn verify_economic_replay(lhs: &SettlementContinuity, rhs: &SettlementContinuity) -> bool {
    lhs.records == rhs.records
}
