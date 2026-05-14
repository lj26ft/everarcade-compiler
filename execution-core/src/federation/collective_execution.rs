use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectiveExecutionPlan {
    pub execution_root: String,
    pub steps: Vec<String>,
}

impl CollectiveExecutionPlan {
    pub fn new(mut steps: Vec<String>) -> Self {
        steps.sort();
        let execution_root = hash_bytes(steps.join("|").as_bytes());
        Self {
            execution_root,
            steps,
        }
    }
}
