use contract_api::{
    Contract,
    ContractError,
    State,
    StateChange,
};

use serde_json::Value;

use increment_contract::IncrementContract;
use set_contract::SetContract;

pub fn execute_action(
    action: &str,
    state: &mut State,
    payload: Value,
) -> Result<Vec<StateChange>, ContractError> {
    match action {
        "set" => {
            let p = SetContract::decode(payload)?;

            SetContract::execute(state, p)
        }

        "increment" => {
            let p =
                IncrementContract::decode(payload)?;

            IncrementContract::execute(state, p)
        }

        _ => Err(ContractError(format!(
            "unknown action: {}",
            action
        ))),
    }
}
