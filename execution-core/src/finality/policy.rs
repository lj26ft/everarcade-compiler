use serde::{Deserialize, Serialize};

use super::{errors::FinalityError, finalize::FinalizationReport};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityPolicy {
    pub require_quorum: bool,
}

pub fn verify_finality_policy(
    policy: &FinalityPolicy,
    report: &FinalizationReport,
) -> Result<(), FinalityError> {
    if policy.require_quorum && !report.quorum_reached {
        return Err(FinalityError::QuorumNotReached);
    }
    Ok(())
}
