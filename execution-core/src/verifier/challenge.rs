#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChallengeReason {
    ReceiptHashMismatch,
    StateRootMismatch,
}

#[derive(Debug, Clone)]
pub struct Challenge {
    pub challenger: String,
    pub challenged_receipt: String,
    pub reason: ChallengeReason,
}
