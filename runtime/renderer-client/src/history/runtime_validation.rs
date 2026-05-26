use super::{continuity_chain::ReplayContinuityChain, corruption::HistoricalCorruptionMatrix};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalValidationStage {
    pub stage: String,
    pub passed: bool,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalValidationReport {
    pub generated_at_unix: u64,
    pub stages: Vec<HistoricalValidationStage>,
    pub success: bool,
}

#[derive(Debug, Default)]
pub struct HistoricalRuntimeValidationEngine;

impl HistoricalRuntimeValidationEngine {
    pub fn execute(
        generated_at_unix: u64,
        continuity_ok: bool,
        corruption_matrix: &HistoricalCorruptionMatrix,
    ) -> HistoricalValidationReport {
        let corruption = corruption_matrix.evaluate();
        let stages = vec![
            HistoricalValidationStage {
                stage: "continuity_chain".into(),
                passed: continuity_ok,
                details: if continuity_ok {
                    "append-only chain verified".into()
                } else {
                    "continuity mismatch".into()
                },
            },
            HistoricalValidationStage {
                stage: "corruption_matrix".into(),
                passed: corruption.accepted,
                details: format!("rejected={:?}", corruption.rejected_scenarios),
            },
        ];
        let success = stages.iter().all(|s| s.passed);
        HistoricalValidationReport {
            generated_at_unix,
            stages,
            success,
        }
    }

    pub fn verify_manifest_chain() -> bool {
        ReplayContinuityChain::verify_append_only(&[])
    }
}
