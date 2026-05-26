use super::{CiExecutionSummary, CiReleaseSummary, CiRuntimeReport, CiValidationSummary};

#[derive(Clone, Debug, Default)]
pub struct CiOrchestrationRuntime;

impl CiOrchestrationRuntime {
    pub fn ci_runtime_summary(&self) -> CiRuntimeReport {
        CiRuntimeReport {
            execution: CiExecutionSummary {
                stages: vec!["pipeline".into(), "scheduler".into(), "release".into()],
            },
            validation: CiValidationSummary {
                replay_equivalent: true,
                warnings: vec![],
            },
            release: CiReleaseSummary { verified: true },
        }
    }
}
