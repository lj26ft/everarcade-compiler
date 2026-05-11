use super::epoch::ProtocolEpoch;

pub fn is_allowed_transition(from: &ProtocolEpoch, to: &ProtocolEpoch) -> bool {
    to.epoch_id == from.epoch_id + 1
}
