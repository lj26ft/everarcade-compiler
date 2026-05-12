#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoordinationGraph {
    pub coordinators: Vec<String>,
}

impl CoordinationGraph {
    pub fn canonical(mut self) -> Self {
        self.coordinators.sort();
        self.coordinators.dedup();
        self
    }
}
