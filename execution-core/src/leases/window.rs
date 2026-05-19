use serde::{Deserialize, Serialize};

use super::errors::LeaseError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseWindow {
    pub start_tick: u64,
    pub end_tick: u64,
}

pub fn verify_lease_window(
    current: &LeaseWindow,
    previous: Option<&LeaseWindow>,
) -> Result<(), LeaseError> {
    if current.start_tick > current.end_tick {
        return Err(LeaseError::InvalidWindow);
    }
    if let Some(prev) = previous {
        if current.start_tick <= prev.start_tick || current.end_tick <= prev.end_tick {
            return Err(LeaseError::NonMonotonicWindow);
        }
        if current.start_tick <= prev.end_tick {
            return Err(LeaseError::OverlappingWindow);
        }
    }
    Ok(())
}
