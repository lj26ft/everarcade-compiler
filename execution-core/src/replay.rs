use crate::{hash::hash_bytes, types::Hash};

pub struct ExecutionRecord {
    pub input: Vec<u8>,
    pub output: Vec<u8>,
    pub hash: Hash,
}

pub fn record(input: &[u8], output: &[u8]) -> ExecutionRecord {
    let mut combined = Vec::new();
    combined.extend_from_slice(input);
    combined.extend_from_slice(output);

    let hash = hash_bytes(&combined);

    ExecutionRecord {
        input: input.to_vec(),
        output: output.to_vec(),
        hash,
    }
}

pub fn verify(record: &ExecutionRecord) -> bool {
    let mut combined = Vec::new();
    combined.extend_from_slice(&record.input);
    combined.extend_from_slice(&record.output);

    hash_bytes(&combined) == record.hash
}
