use serde::{Deserialize, Serialize};

use super::errors::FinalityError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizationWindow {
    pub start_tick: u64,
    pub end_tick: u64,
}

pub fn verify_finalization_window(
    current: &FinalizationWindow,
    previous: Option<&FinalizationWindow>,
) -> Result<(), FinalityError> {
    if current.start_tick > current.end_tick {
        return Err(FinalityError::InvalidWindow);
    }
    if let Some(prev) = previous {
        if current.start_tick <= prev.end_tick {
            return Err(FinalityError::OverlappingWindow);
        }
    }
    Ok(())
}
