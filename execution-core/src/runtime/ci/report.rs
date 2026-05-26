#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiExecutionSummary {
    pub stages: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiValidationSummary {
    pub replay_equivalent: bool,
    pub warnings: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiReleaseSummary {
    pub verified: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CiRuntimeReport {
    pub execution: CiExecutionSummary,
    pub validation: CiValidationSummary,
    pub release: CiReleaseSummary,
}
