use contract_api::{Contract, ContractError, State, StateChange};

use serde::Deserialize;
use serde_json::Value;

pub struct SetContract;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetPayload {
    pub key: String,
    pub value: String,
}

impl Contract for SetContract {
    type Payload = SetPayload;

    fn decode(value: Value) -> Result<Self::Payload, ContractError> {
        serde_json::from_value(value)
            .map_err(|e| ContractError(format!("invalid set payload: {}", e)))
    }

    fn execute(
        state: &mut State,
        payload: Self::Payload,
    ) -> Result<Vec<StateChange>, ContractError> {
        let before = state.get(&payload.key).cloned().unwrap_or_default();

        state.insert(payload.key.clone(), payload.value.clone());

        Ok(vec![StateChange {
            key: payload.key,
            before,
            after: payload.value,
        }])
    }
}
