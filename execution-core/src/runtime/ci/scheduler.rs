#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationBatch {
    pub id: String,
    pub partition: String,
    pub priority: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationPartitionSchedule {
    pub partitions: Vec<String>,
    pub batches: Vec<ValidationBatch>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationExecutionWindow {
    pub start_tick: u64,
    pub end_tick: u64,
}
#[derive(Clone, Debug, Default)]
pub struct CiSchedulerRuntime;
impl CiSchedulerRuntime {
    pub fn schedule(&self, mut batches: Vec<ValidationBatch>) -> ValidationPartitionSchedule {
        batches.sort_by_key(|b| (b.priority, b.partition.clone(), b.id.clone()));
        let mut partitions: Vec<String> = batches.iter().map(|b| b.partition.clone()).collect();
        partitions.sort();
        partitions.dedup();
        ValidationPartitionSchedule {
            partitions,
            batches,
        }
    }
}
