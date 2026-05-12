#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionBatch {
    pub batch_index: usize,
    pub node_ids: Vec<String>,
}
