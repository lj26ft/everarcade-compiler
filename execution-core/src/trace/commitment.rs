use crate::{
    hashing::hash_bytes,
    trace::{serialization::serialize_trace, trace::ExecutionTrace},
};

pub fn trace_root(trace: &ExecutionTrace) -> String {
    hash_bytes(&serialize_trace(trace))
}

pub fn transition_commitments(trace: &ExecutionTrace) -> Vec<String> {
    trace
        .transitions
        .iter()
        .map(|t| hash_bytes(&bincode::serialize(t).expect("transition serialization must succeed")))
        .collect()
}

pub fn replay_commitment(trace: &ExecutionTrace) -> String {
    hash_bytes(trace_root(trace).as_bytes())
}
