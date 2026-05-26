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
