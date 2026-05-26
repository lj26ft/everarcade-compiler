#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayBranch { pub branch_id: String, pub parent_root: String, pub root: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayForkProof { pub expected_parent_root: String }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayBranchContinuity { pub contiguous: bool }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayForkVerification { pub valid: bool, pub reason: String }
impl ReplayForkVerification { pub fn verify(branch: &ReplayBranch, proof: &ReplayForkProof) -> Self { let valid = branch.parent_root == proof.expected_parent_root; Self { valid, reason: if valid { "ok".into() } else { "divergence".into() } } } }
