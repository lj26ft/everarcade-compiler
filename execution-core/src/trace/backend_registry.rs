use std::{collections::BTreeMap, sync::Arc};

use crate::trace::{backend::ProofBackend, backend_mock::MockProofBackend};

pub struct ProofBackendRegistry {
    backends: BTreeMap<String, Arc<dyn ProofBackend>>,
}

impl ProofBackendRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            backends: BTreeMap::new(),
        };
        registry.register(Arc::new(MockProofBackend));
        registry
    }

    pub fn register(&mut self, backend: Arc<dyn ProofBackend>) {
        self.backends
            .insert(backend.backend_id().to_string(), backend);
    }

    pub fn get(&self, backend_id: &str) -> Option<Arc<dyn ProofBackend>> {
        self.backends.get(backend_id).cloned()
    }
}
