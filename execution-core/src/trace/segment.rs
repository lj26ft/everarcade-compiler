use crate::trace::trace::ExecutionTrace;

pub fn segment_trace(trace: &ExecutionTrace, segment_size: usize) -> Vec<ExecutionTrace> {
    if segment_size == 0 {
        return vec![trace.clone()];
    }

    trace
        .transitions
        .chunks(segment_size)
        .enumerate()
        .map(|(idx, transitions)| {
            let mut segmented = trace.clone();
            segmented.trace_id = format!("{}-segment-{}", segmented.trace_id, idx);
            segmented.transitions = transitions.to_vec();
            segmented
        })
        .collect()
}
