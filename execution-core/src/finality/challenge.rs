#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionChallenge {
    pub challenger: String,
    pub challenged_receipt_hash: String,
    pub canonical_receipt_hash: String,
}

impl ExecutionChallenge {
    pub fn is_valid(&self) -> bool {
        self.challenged_receipt_hash != self.canonical_receipt_hash
    }
}
