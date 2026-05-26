#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationStageResult {
    pub stage_id: String,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationExecutionSummary {
    pub total_stages: usize,
    pub passed_stages: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReportManifest {
    pub stages: Vec<ValidationStageResult>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeValidationExecution {
    pub timestamp_utc: String,
    pub replay_equivalence: bool,
    pub warning_gate_passed: bool,
    pub security_gate_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeValidationFailure {
    pub stage_id: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeGeneratedValidationReport {
    pub execution: RuntimeValidationExecution,
    pub restoration_passed: bool,
    pub partition_diagnostics: String,
    pub load_diagnostics: String,
}

#[derive(Clone, Debug, Default)]
pub struct ValidationReportRuntime;

impl ValidationReportRuntime {
    pub fn summarize(&self, manifest: &ValidationReportManifest) -> ValidationExecutionSummary {
        let passed = manifest.stages.iter().filter(|s| s.passed).count();
        ValidationExecutionSummary {
            total_stages: manifest.stages.len(),
            passed_stages: passed,
        }
    }
}
