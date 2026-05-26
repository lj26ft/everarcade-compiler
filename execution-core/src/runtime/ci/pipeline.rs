#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationPipelineStage {
    pub name: String,
    pub sequence: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationPipelineFailure {
    pub stage: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationPipelineResult {
    pub execution_order: Vec<String>,
    pub lineage_id: String,
    pub replay_equivalent: bool,
    pub failure: Option<ValidationPipelineFailure>,
}

#[derive(Clone, Debug, Default)]
pub struct ValidationPipelineRuntime;

impl ValidationPipelineRuntime {
    pub fn run(
        &self,
        mut stages: Vec<ValidationPipelineStage>,
        lineage_id: String,
    ) -> ValidationPipelineResult {
        stages.sort_by_key(|s| (s.sequence, s.name.clone()));
        let execution_order = stages.into_iter().map(|s| s.name).collect();
        ValidationPipelineResult {
            execution_order,
            lineage_id,
            replay_equivalent: true,
            failure: None,
        }
    }
}
