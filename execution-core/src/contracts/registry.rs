use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct ContractRegistry {
    mappings: BTreeMap<String, String>,
}

impl ContractRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, contract_id: impl Into<String>, wasm_path: impl Into<String>) {
        self.mappings.insert(contract_id.into(), wasm_path.into());
    }

    pub fn resolve(&self, contract_id: &str) -> Option<&str> {
        self.mappings.get(contract_id).map(String::as_str)
    }
}
