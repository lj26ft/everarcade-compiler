use super::store::AiMemoryStore;

pub fn memory_is_append_only(store: &AiMemoryStore) -> bool {
    store
        .entries
        .windows(2)
        .all(|w| w[0].entity_id <= w[1].entity_id || w[0].sequence <= w[1].sequence)
        && store.entries.iter().all(|e| !e.replay_root.is_empty())
}

pub fn memory_equivalent(a: &AiMemoryStore, b: &AiMemoryStore) -> bool {
    a == b
}
