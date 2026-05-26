#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseArtifact {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseManifest {
    pub artifacts: Vec<SovereignReleaseArtifact>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SovereignReleaseVerification {
    pub reproducible: bool,
}
#[derive(Clone, Debug, Default)]
pub struct SovereignReleaseRuntime;
use super::{
    ValidationCheckpointRuntime, ValidationDagExecution, ValidationDagRuntime,
    ValidationReportManifest, ValidationReportRuntime,
};

#[derive(Clone, Debug, Default)]
pub struct ValidationRuntime;

impl ValidationRuntime {
    pub fn run(
        &self,
        dag: &ValidationDagRuntime,
        checkpoints: &mut ValidationCheckpointRuntime,
    ) -> Result<(ValidationDagExecution, ValidationReportManifest), String> {
        let execution = dag.execute(checkpoints)?;
        let stages: Vec<super::ValidationStageResult> = execution
            .ordered_stages
            .iter()
            .map(|stage_id| super::ValidationStageResult {
                stage_id: stage_id.clone(),
                passed: true,
            })
            .collect();
        let _summary = ValidationReportRuntime.summarize(&ValidationReportManifest {
            stages: stages.clone(),
        });
        Ok((execution, ValidationReportManifest { stages }))
    }
}
