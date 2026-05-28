use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Archetype {
    pub component_names: Vec<String>,
}

impl Archetype {
    pub fn canonical(mut component_names: Vec<String>) -> Self {
        component_names.sort();
        Self { component_names }
    }
}
