pub fn transition_multiplier(epoch_from: u64, epoch_to: u64) -> u64 {
    if epoch_to <= epoch_from { return 100; }
    let delta = epoch_to - epoch_from;
    100 + delta.min(20)
}
