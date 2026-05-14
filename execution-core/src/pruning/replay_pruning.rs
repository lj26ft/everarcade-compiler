use super::pruning_policy::PruningPolicy;

pub fn pruneable_epochs(current_epoch: u64, policy: &PruningPolicy) -> Vec<u64> {
    if current_epoch <= policy.retention.keep_last_epochs {
        return vec![];
    }
    (0..=(current_epoch - policy.retention.keep_last_epochs - 1)).collect()
}
