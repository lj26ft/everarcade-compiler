use crate::federation::{
    checkpoint::FederationCheckpoint,
    replay_verify::{verify_replay_equivalence, ReplayVerificationInput},
    settlement::FederationSettlementJournal,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FederationRecoveryState {
    pub checkpoints: Vec<FederationCheckpoint>,
    pub settlement_journal: FederationSettlementJournal,
    pub restored_state_root: String,
}

pub fn recover_continuity(
    state: &FederationRecoveryState,
    expected: &ReplayVerificationInput,
) -> Result<(), String> {
    let last = state
        .checkpoints
        .last()
        .ok_or_else(|| "missing checkpoint".to_string())?;
    let recovered = ReplayVerificationInput {
        receipt_hash: last.receipt_root.clone(),
        state_root: state.restored_state_root.clone(),
        replay_root: last.replay_root.clone(),
        settlement_root: last.settlement_root.clone(),
    };
    verify_replay_equivalence(&recovered, expected)
}
