use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentValue {
    pub name: String,
    pub value: i64,
    pub authority: String,
}

impl ComponentValue {
    pub fn new(name: impl Into<String>, value: i64, authority: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value,
            authority: authority.into(),
        }
    }
}
