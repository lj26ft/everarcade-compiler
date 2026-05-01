use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub type State = BTreeMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateChange {
    pub key: String,
    pub before: String,
    pub after: String,
}
