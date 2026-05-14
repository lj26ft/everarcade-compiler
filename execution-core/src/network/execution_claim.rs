#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionClaim {
    pub package_id: String,
    pub node_id: String,
    pub receipt_hash: String,
    pub claimed_epoch: u64,
}

#[derive(Debug, Default)]
pub struct ExecutionClaimValidator;

impl ExecutionClaimValidator {
    pub fn validate(
        claim: &ExecutionClaim,
        expected_epoch: u64,
        expected_receipt_hash: &str,
    ) -> bool {
        claim.claimed_epoch == expected_epoch && claim.receipt_hash == expected_receipt_hash
    }
}
