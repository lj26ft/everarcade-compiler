use serde::{Deserialize, Serialize};

use super::errors::DivergenceError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergenceWindow {
    pub start_tick: u64,
    pub end_tick: u64,
}

pub fn verify_divergence_window(
    window: &DivergenceWindow,
    existing_windows: &[DivergenceWindow],
) -> Result<(), DivergenceError> {
    if window.start_tick > window.end_tick {
        return Err(DivergenceError::InvalidWindow);
    }
    for existing in existing_windows {
        let overlap =
            window.start_tick <= existing.end_tick && existing.start_tick <= window.end_tick;
        if overlap {
            return Err(DivergenceError::OverlappingWindow);
        }
    }
    Ok(())
}
