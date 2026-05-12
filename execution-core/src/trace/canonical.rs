use crate::trace::trace::ExecutionTrace;

pub fn canonicalize_trace(trace: &mut ExecutionTrace) {
    trace.nodes.sort_by(|a, b| a.index.cmp(&b.index).then(a.node_id.cmp(&b.node_id)));
    trace.transitions
        .sort_by(|a, b| a.index.cmp(&b.index).then(a.transition_id.cmp(&b.transition_id)));
}
