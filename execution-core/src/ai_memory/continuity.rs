use super::store::AiMemoryStore;

pub fn lineage_roots(store: &AiMemoryStore, entity_id: &str) -> Vec<String> {
    store
        .entries_for(entity_id)
        .into_iter()
        .map(|e| e.replay_root)
        .collect()
}
