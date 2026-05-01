use contract_api::{
    Contract,
    ContractError,
    State,
    StateChange,
};

use serde::Deserialize;
use serde_json::Value;

pub struct IncrementContract;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncrementPayload {
    pub key: String,
}

impl Contract for IncrementContract {
    type Payload = IncrementPayload;

    fn decode(
        value: Value,
    ) -> Result<Self::Payload, ContractError> {
        serde_json::from_value(value)
            .map_err(|e| {
                ContractError(format!(
                    "invalid increment payload: {}",
                    e
                ))
            })
    }

    fn execute(
        state: &mut State,
        payload: Self::Payload,
    ) -> Result<Vec<StateChange>, ContractError> {
        let current = state
            .get(&payload.key)
            .cloned()
            .unwrap_or_else(|| "0".to_string());

        let value: i64 = current.parse().map_err(|_| {
            ContractError(format!(
                "value is not numeric: {}",
                current
            ))
        })?;

        let next = value + 1;

        state.insert(
            payload.key.clone(),
            next.to_string(),
        );

        Ok(vec![StateChange {
            key: payload.key,
            before: current,
            after: next.to_string(),
        }])
    }
}
