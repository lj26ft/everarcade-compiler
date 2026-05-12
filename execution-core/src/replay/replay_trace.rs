use super::replay_step::TraceStep;

pub fn validate_step_link(prev: Option<&TraceStep>, current: &TraceStep) -> bool {
    match prev {
        None => current.logical_index == 0,
        Some(prev_step) => {
            prev_step.logical_index + 1 == current.logical_index
                && prev_step.next_state_root == current.prior_state_root
        }
    }
}
