use super::epoch::ExecutionEpoch;

pub fn validate_epoch_transition(previous: &ExecutionEpoch, next: &ExecutionEpoch) -> bool {
    previous.epoch_index + 1 == next.epoch_index && previous.end_receipt == next.start_receipt
}
