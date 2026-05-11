use crate::hashing;

pub fn load_contract_bytes(path: &str) -> std::io::Result<Vec<u8>> { std::fs::read(path) }

pub fn compute_contract_hash(wasm_bytes: &[u8]) -> String { hashing::compute_contract_hash(wasm_bytes) }
