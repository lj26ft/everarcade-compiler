use serde::Serialize;

use super::{reports::RuntimeValidationReport, RuntimeMetrics};

fn canonical_eq<T: Serialize>(left: &T, right: &T) -> Result<(), String> {
    let l = crate::canonical::encoding::canonical_encode(left).map_err(|e| e.to_string())?;
    let r = crate::canonical::encoding::canonical_encode(right).map_err(|e| e.to_string())?;
    if l == r {
        Ok(())
    } else {
        Err("canonical mismatch".into())
    }
}

pub fn assert_runtime_equivalence(
    left: &RuntimeValidationReport,
    right: &RuntimeValidationReport,
) -> Result<(), String> {
    canonical_eq(left, right)
}

pub fn assert_epoch_equivalence(
    left: &RuntimeMetrics,
    right: &RuntimeMetrics,
) -> Result<(), String> {
    canonical_eq(&left.epoch, &right.epoch)
}

pub fn assert_replay_equivalence(
    left: &RuntimeMetrics,
    right: &RuntimeMetrics,
) -> Result<(), String> {
    canonical_eq(&left.replay, &right.replay)
}

pub fn assert_restoration_equivalence(
    left: &RuntimeMetrics,
    right: &RuntimeMetrics,
) -> Result<(), String> {
    canonical_eq(&left.restoration, &right.restoration)
}

pub fn assert_lane_equivalence(
    left: &RuntimeMetrics,
    right: &RuntimeMetrics,
) -> Result<(), String> {
    canonical_eq(&left.lane, &right.lane)
}

pub fn assert_snapshot_equivalence(
    left: &RuntimeMetrics,
    right: &RuntimeMetrics,
) -> Result<(), String> {
    canonical_eq(&left.snapshot, &right.snapshot)
}

pub fn assert_witness_equivalence(
    left: &RuntimeMetrics,
    right: &RuntimeMetrics,
) -> Result<(), String> {
    canonical_eq(&left.witness, &right.witness)
}
