use crate::trace::{backend::ExecutionProof, trace::ExecutionTrace};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraceSchemaCompatibility {
    Compatible,
    EpochMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendCompatibility {
    Compatible,
    BackendMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub trace_schema: TraceSchemaCompatibility,
    pub backend: BackendCompatibility,
}

pub fn validate_compatibility(trace: &ExecutionTrace, proof: &ExecutionProof, expected_epoch: u64) -> CompatibilityReport {
    let trace_schema = if trace.epoch_id == expected_epoch {
        TraceSchemaCompatibility::Compatible
    } else {
        TraceSchemaCompatibility::EpochMismatch
    };

    let backend = if proof.backend_id.is_empty() {
        BackendCompatibility::BackendMismatch
    } else {
        BackendCompatibility::Compatible
    };

    CompatibilityReport { trace_schema, backend }
}
