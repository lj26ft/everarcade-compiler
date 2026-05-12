use crate::trace::trace::ExecutionTrace;

pub fn replay_trace(trace: &ExecutionTrace) -> ExecutionTrace {
    trace.clone()
}

pub fn replay_matches(trace: &ExecutionTrace, replayed: &ExecutionTrace) -> bool {
    trace == replayed
}
