use super::{
    economy::CivilizationEconomy, governance::GovernanceState, runtime::CivilizationRuntimeState,
    settlement::Settlement,
};
pub fn evolve_civilization(
    state: &CivilizationRuntimeState,
    input: &str,
) -> CivilizationRuntimeState {
    let tick = state.tick + 1;
    let settlements = state
        .settlements
        .iter()
        .map(|s| Settlement {
            settlement_id: s.settlement_id.clone(),
            population: s.population + 1,
            resource_root: format!("settlement:{}:resources:{tick}:{input}", s.settlement_id),
        })
        .collect();
    let economy = CivilizationEconomy {
        ledger_root: format!(
            "civilization:{}:ledger:{tick}:{input}",
            state.civilization_id
        ),
        supply: state.economy.supply + tick,
    };
    let governance = GovernanceState {
        governance_root: format!("civilization:{}:governance:{tick}", state.civilization_id),
        epoch: tick,
    };
    let continuity_root = format!(
        "civilization:{}:continuity:{tick}:{}:{}",
        state.civilization_id, economy.ledger_root, governance.governance_root
    );
    CivilizationRuntimeState {
        civilization_id: state.civilization_id.clone(),
        tick,
        settlements,
        economy,
        governance,
        replay_tip: format!(
            "civilization:{}:replay:{tick}:{input}",
            state.civilization_id
        ),
        continuity_root,
    }
}
