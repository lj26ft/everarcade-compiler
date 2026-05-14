use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::BTreeMap;

pub type State = BTreeMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub key: String,
    pub before: String,
    pub after: String,
}

#[derive(Debug)]
pub struct ContractError(pub String);

pub trait Contract {
    type Payload;

    fn decode(value: Value) -> Result<Self::Payload, ContractError>;

    fn execute(
        state: &mut State,
        payload: Self::Payload,
    ) -> Result<Vec<StateChange>, ContractError>;
}
