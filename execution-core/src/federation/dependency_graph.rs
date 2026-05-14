use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencyGraph {
    pub dependencies: BTreeMap<String, BTreeSet<String>>,
}

impl DependencyGraph {
    pub fn dependency_batch(&self, key: &str) -> Vec<String> {
        self.dependencies
            .get(key)
            .map(|d| d.iter().cloned().collect())
            .unwrap_or_default()
    }
}
