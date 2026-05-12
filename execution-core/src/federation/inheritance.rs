use crate::hashing::hash_bytes;

pub fn inheritance_root(parent: &str, successor: &str) -> String {
    hash_bytes(format!("{parent}:{successor}").as_bytes())
}
