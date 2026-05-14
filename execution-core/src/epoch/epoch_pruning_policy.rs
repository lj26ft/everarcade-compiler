#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpochPruningPolicy {
    pub min_retained_epochs: u64,
}
