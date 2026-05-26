#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValidationStageNode {
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationStageDependency {
    pub from: String,
    pub to: String,
}
