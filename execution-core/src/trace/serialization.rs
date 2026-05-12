use crate::trace::{canonical::canonicalize_trace, trace::ExecutionTrace};

pub fn serialize_trace(trace: &ExecutionTrace) -> Vec<u8> {
    let mut canonical = trace.clone();
    canonicalize_trace(&mut canonical);
    bincode::serialize(&canonical).expect("canonical trace serialization must succeed")
}

pub fn deserialize_trace(bytes: &[u8]) -> Result<ExecutionTrace, bincode::Error> {
    bincode::deserialize(bytes)
}
