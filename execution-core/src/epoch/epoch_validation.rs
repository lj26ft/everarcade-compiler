use super::{epoch::ExecutionEpoch, epoch_root::compute_epoch_root};

pub fn validate_epoch(epoch: &ExecutionEpoch) -> bool {
    compute_epoch_root(epoch) == epoch.epoch_root
}
