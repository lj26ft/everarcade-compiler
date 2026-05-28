use super::{
    economy::CivilizationEconomy, evolution::evolve_civilization, governance::GovernanceState,
    settlement::Settlement, validation::validate_civilization_runtime,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CivilizationRuntimeState {
    pub civilization_id: String,
    pub tick: u64,
    pub settlements: Vec<Settlement>,
    pub economy: CivilizationEconomy,
    pub governance: GovernanceState,
    pub replay_tip: String,
    pub continuity_root: String,
}
impl CivilizationRuntimeState {
    pub fn genesis(id: &str) -> Self {
        let economy = CivilizationEconomy::genesis(id);
        let governance = GovernanceState::genesis(id);
        Self {
            civilization_id: id.into(),
            tick: 0,
            settlements: vec![Settlement::new("capital")],
            economy,
            governance,
            replay_tip: format!("civilization:{id}:replay:0"),
            continuity_root: format!(
                "civilization:{id}:continuity:0:{}:{}",
                CivilizationEconomy::genesis(id).ledger_root,
                GovernanceState::genesis(id).governance_root
            ),
        }
    }
    pub fn tick(&mut self, input: &str) -> Result<(), &'static str> {
        let next = evolve_civilization(self, input);
        if validate_civilization_runtime(&next) {
            *self = next;
            Ok(())
        } else {
            Err("civilization divergence rejected")
        }
    }
}
