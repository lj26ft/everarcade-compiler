pub fn combine_roots(parts: &[String]) -> String { crate::hashing::hash_bytes(parts.join(":").as_bytes()) }
