use serde::{Deserialize, Serialize};

use super::{errors::AuthorityError, handoff::AuthorityHandoff};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationPolicy {
    pub allow_handoff: bool,
}

pub fn verify_rotation_policy(
    policy: &RotationPolicy,
    handoff: &AuthorityHandoff,
) -> Result<(), AuthorityError> {
    if !policy.allow_handoff && handoff.from != handoff.to {
        return Err(AuthorityError::HandoffDisabled);
    }
    Ok(())
}
