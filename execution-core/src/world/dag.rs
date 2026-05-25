use crate::{canonical::encoding::canonical_encode, hashing::hash_bytes};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExecutionShard(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExecutionDependency(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ExecutionPartitionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub id: String,
    pub shard: ExecutionShard,
    pub phase: ExecutionPhase,
    pub partition: ExecutionPartitionId,
    pub dependencies: Vec<ExecutionDependency>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEdge {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExecutionPhase {
    Prepare,
    Execute,
    Finalize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPartition {
    pub id: ExecutionPartitionId,
    pub mutation_keys: Vec<String>,
    pub node_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartitionRoot(pub String);

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionGraph {
    pub nodes: BTreeMap<String, ExecutionNode>,
}

impl ExecutionGraph {
    pub fn add_node(&mut self, mut node: ExecutionNode) {
        node.dependencies.sort();
        self.nodes.insert(node.id.clone(), node);
    }
    pub fn add_edge(&mut self, edge: ExecutionEdge) {
        if let Some(node) = self.nodes.get_mut(&edge.to) {
            node.dependencies.push(ExecutionDependency(edge.from));
            node.dependencies.sort();
            node.dependencies.dedup();
        }
    }
    pub fn canonical_order(&self) -> Result<Vec<String>, String> {
        let mut indegree: BTreeMap<String, usize> =
            self.nodes.keys().map(|k| (k.clone(), 0)).collect();
        let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (id, node) in &self.nodes {
            for dep in &node.dependencies {
                if indegree.contains_key(id) {
                    *indegree.get_mut(id).unwrap() += 1;
                    out.entry(dep.0.clone()).or_default().push(id.clone());
                }
            }
        }
        for v in out.values_mut() {
            v.sort();
        }
        let mut ready: BTreeSet<String> = indegree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(k, _)| k.clone())
            .collect();
        let mut ordered = Vec::new();
        while let Some(next) = ready.pop_first() {
            ordered.push(next.clone());
            if let Some(children) = out.get(&next) {
                for c in children {
                    let d = indegree.get_mut(c).ok_or("missing node")?;
                    *d -= 1;
                    if *d == 0 {
                        ready.insert(c.clone());
                    }
                }
            }
        }
        if ordered.len() != self.nodes.len() {
            return Err("cycle detected".into());
        }
        Ok(ordered)
    }
    pub fn stable_hash(&self) -> Result<String, String> {
        Ok(hash_bytes(
            &canonical_encode(self).map_err(|e| e.to_string())?,
        ))
    }
    pub fn partition_root(partitions: &[ExecutionPartition]) -> Result<PartitionRoot, String> {
        let mut parts = partitions.to_vec();
        for part in &mut parts {
            part.mutation_keys.sort();
            part.node_ids.sort();
        }
        parts.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(PartitionRoot(hash_bytes(
            &canonical_encode(&parts).map_err(|e| e.to_string())?,
        )))
    }
}
