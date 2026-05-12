use crate::{hashing::hash_bytes, trace::{commitment::transition_commitments, trace::ExecutionTrace}};

pub fn merkle_trace_root(trace: &ExecutionTrace) -> String {
    let mut leaves = transition_commitments(trace);
    if leaves.is_empty() {
        return hash_bytes(b"empty-merkle-trace");
    }

    while leaves.len() > 1 {
        let mut next = Vec::with_capacity((leaves.len() + 1) / 2);
        for pair in leaves.chunks(2) {
            let right = if pair.len() == 2 { &pair[1] } else { &pair[0] };
            next.push(hash_bytes(format!("{}{}", pair[0], right).as_bytes()));
        }
        leaves = next;
    }
    leaves[0].clone()
}
