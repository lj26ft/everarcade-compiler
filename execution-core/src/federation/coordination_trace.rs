use crate::hashing::hash_bytes;

pub fn trace_root(entries: &[String]) -> String {
    hash_bytes(entries.join("|").as_bytes())
}
