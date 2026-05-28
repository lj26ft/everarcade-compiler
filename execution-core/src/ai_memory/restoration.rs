use super::{runtime::AiMemoryRuntime, store::AiMemoryStore};

pub fn restore_memory(store: &AiMemoryStore) -> AiMemoryRuntime {
    AiMemoryRuntime {
        store: store.clone(),
    }
}
