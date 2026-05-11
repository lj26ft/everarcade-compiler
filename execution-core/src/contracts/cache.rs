use std::collections::BTreeMap;

#[derive(Default)]
pub struct ContractCache {
    modules: BTreeMap<String, Vec<u8>>,
}

impl ContractCache {
    pub fn get(&self, key: &str) -> Option<&[u8]> { self.modules.get(key).map(Vec::as_slice) }

    pub fn insert(&mut self, key: impl Into<String>, module: Vec<u8>) {
        self.modules.insert(key.into(), module);
    }
}
