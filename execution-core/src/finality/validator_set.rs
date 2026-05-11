#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorSet {
    pub epoch_id: u64,
    pub verifiers: Vec<String>,
}

impl ValidatorSet {
    pub fn contains(&self, verifier_id: &str) -> bool {
        self.verifiers.iter().any(|v| v == verifier_id)
    }
}
