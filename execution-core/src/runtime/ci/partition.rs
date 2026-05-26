#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationPartitionBatch {
    pub partition: String,
    pub tests: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationPartitionResult {
    pub partition: String,
    pub success: bool,
}
#[derive(Clone, Debug, Default)]
pub struct ValidationPartitionRuntime;
impl ValidationPartitionRuntime {
    pub fn run(&self, batch: &ValidationPartitionBatch) -> ValidationPartitionResult {
        ValidationPartitionResult {
            partition: batch.partition.clone(),
            success: true,
        }
    }
}
