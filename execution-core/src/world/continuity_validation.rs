use super::RuntimeMetrics;

pub fn validate_checkpoint_lineage_integrity(metrics: &RuntimeMetrics) -> Result<(), String> {
    if metrics.epoch.checkpoint_lineage_root.is_empty() {
        Err("missing checkpoint lineage root".into())
    } else {
        Ok(())
    }
}

pub fn validate_epoch_continuity_integrity(metrics: &RuntimeMetrics) -> Result<(), String> {
    if metrics.epoch.epoch_count == 0 {
        Err("missing epoch continuity".into())
    } else {
        Ok(())
    }
}

pub fn validate_window_continuity_integrity(metrics: &RuntimeMetrics) -> Result<(), String> {
    if metrics.replay.replay_window_count == 0 {
        Err("missing replay windows".into())
    } else {
        Ok(())
    }
}

pub fn validate_snapshot_continuity_integrity(metrics: &RuntimeMetrics) -> Result<(), String> {
    if metrics.snapshot.snapshot_chain_root.is_empty() {
        Err("missing snapshot continuity".into())
    } else {
        Ok(())
    }
}

pub fn validate_event_continuity_integrity(metrics: &RuntimeMetrics) -> Result<(), String> {
    if metrics.event.aggregated_event_root.is_empty() {
        Err("missing event continuity".into())
    } else {
        Ok(())
    }
}

pub fn validate_witness_continuity_integrity(metrics: &RuntimeMetrics) -> Result<(), String> {
    if metrics.witness.aggregated_witness_root.is_empty() {
        Err("missing witness continuity".into())
    } else {
        Ok(())
    }
}

pub fn validate_replay_continuity_integrity(metrics: &RuntimeMetrics) -> Result<(), String> {
    if !metrics.replay.replay_equivalence {
        Err("replay divergence".into())
    } else {
        Ok(())
    }
}
