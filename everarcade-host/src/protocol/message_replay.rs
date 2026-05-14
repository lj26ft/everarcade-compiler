use std::collections::BTreeSet;

pub fn not_replayed(message_id: &str, seen: &mut BTreeSet<String>) -> bool {
    seen.insert(message_id.to_string())
}
