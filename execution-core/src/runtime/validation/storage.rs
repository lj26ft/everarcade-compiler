#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspacePartition {
    pub id: String,
    pub crates: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspacePartitionManifest {
    pub partitions: Vec<WorkspacePartition>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspacePartitionBoundary {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, Default)]
pub struct WorkspacePartitionRuntime;

impl WorkspacePartitionRuntime {
    pub fn equivalent(
        &self,
        lhs: &WorkspacePartitionManifest,
        rhs: &WorkspacePartitionManifest,
    ) -> bool {
        lhs == rhs
    }
}
