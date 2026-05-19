use serde::{Deserialize, Serialize};

use super::{
    epoch::{verify_epoch_transition, AuthorityEpoch},
    handoff::AuthorityHandoff,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityVerificationReport {
    pub authorized: bool,
    pub epoch_valid: bool,
    pub handoff_valid: bool,
}

pub fn verify_authority_chain(
    epochs: &[AuthorityEpoch],
    handoffs: &[AuthorityHandoff],
) -> AuthorityVerificationReport {
    if epochs.is_empty() {
        return AuthorityVerificationReport {
            authorized: false,
            epoch_valid: false,
            handoff_valid: false,
        };
    }

    let epoch_valid = epochs
        .windows(2)
        .all(|pair| verify_epoch_transition(&pair[0], &pair[1]).is_ok());

    let handoff_valid = handoffs.len() == epochs.len().saturating_sub(1)
        && handoffs
            .iter()
            .zip(epochs.windows(2))
            .all(|(handoff, pair)| {
                handoff.from == pair[0].authority
                    && handoff.to == pair[1].authority
                    && handoff.from != handoff.to
                    && handoff.epoch == pair[1].epoch
                    && handoff.checkpoint_root != [0u8; 32]
                    && handoff.lineage_hash != [0u8; 32]
            });

    AuthorityVerificationReport {
        authorized: epoch_valid && handoff_valid,
        epoch_valid,
        handoff_valid,
    }
}
