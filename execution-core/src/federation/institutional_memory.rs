use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstitutionalMemory { pub archive_root: String }

impl InstitutionalMemory { pub fn new(history: &[String]) -> Self { Self { archive_root: hash_bytes(history.join("|").as_bytes()) } } }
