use crate::hashing::hash_bytes;

pub fn coordination_root(tasks: &[String]) -> String {
    let mut ordered = tasks.to_vec();
    ordered.sort();
    hash_bytes(ordered.join("|").as_bytes())
}
