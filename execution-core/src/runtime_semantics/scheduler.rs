#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheduler;

impl Scheduler {
    pub fn canonical_order<T: Ord + Clone>(mut nodes: Vec<T>) -> Vec<T> {
        nodes.sort();
        nodes
    }
}
