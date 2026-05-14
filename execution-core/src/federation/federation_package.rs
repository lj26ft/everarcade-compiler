use crate::hashing::hash_bytes;

pub fn package_root(parts: &[String]) -> String {
    hash_bytes(parts.join("|").as_bytes())
}
