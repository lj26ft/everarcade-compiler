use std::collections::{BTreeMap, BTreeSet};

use super::{ValidationCheckpointRuntime, ValidationStageDependency, ValidationStageNode};

#[derive(Clone, Debug, Default)]
pub struct ValidationDagExecution {
    pub ordered_stages: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ValidationDagRuntime {
    pub nodes: BTreeSet<String>,
    pub deps: Vec<ValidationStageDependency>,
}

impl ValidationDagRuntime {
    pub fn new(nodes: Vec<ValidationStageNode>, deps: Vec<ValidationStageDependency>) -> Self {
        Self {
            nodes: nodes.into_iter().map(|n| n.id).collect(),
            deps,
        }
    }

    pub fn execute(
        &self,
        checkpoint: &mut ValidationCheckpointRuntime,
    ) -> Result<ValidationDagExecution, String> {
        let mut indegree: BTreeMap<String, usize> =
            self.nodes.iter().map(|n| (n.clone(), 0)).collect();
        for dep in &self.deps {
            if !indegree.contains_key(&dep.from) || !indegree.contains_key(&dep.to) {
                return Err("invalid dependency graph".into());
            }
            *indegree.get_mut(&dep.to).expect("dependency validated") += 1;
        }
        let mut ready: BTreeSet<String> = indegree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(n, _)| n.clone())
            .collect();
        let mut out = Vec::with_capacity(self.nodes.len());
        while let Some(node) = ready.pop_first() {
            checkpoint.mark_completed(node.clone());
            out.push(node.clone());
            for dep in self.deps.iter().filter(|d| d.from == node) {
                let degree = indegree.get_mut(&dep.to).expect("dependency validated");
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(dep.to.clone());
                }
            }
        }
        if out.len() != self.nodes.len() {
            return Err("invalid dependency graph".into());
        }
        Ok(ValidationDagExecution {
            ordered_stages: out,
        })
    }
}
