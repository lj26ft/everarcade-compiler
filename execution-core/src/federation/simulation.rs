use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TopologyKind {
    Linear,
    Star,
    Mesh,
    Partitioned,
    Recovered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationFixture {
    pub federation_id: String,
    pub node_count: usize,
    pub topology: TopologyKind,
    pub replay_lineage: String,
    pub workload: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeValidation {
    pub node_id: String,
    pub replay_root: String,
    pub receipt_root: String,
    pub state_root: String,
    pub equivalent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationDiagnostic {
    pub federation_id: String,
    pub node_id: String,
    pub topology_role: String,
    pub replay_root: String,
    pub archive_root: String,
    pub receipt_root: String,
    pub divergence_detected: bool,
    pub recovery_possible: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum DivergenceType {
    ReceiptMismatch,
    StateDiffMismatch,
    ReplayTruncation,
    ArchiveCorruption,
    TopologyMismatch,
    InvalidRestoration,
    OutOfOrderReplay,
}

#[derive(Debug, Clone)]
pub struct FederationReport {
    pub validations: Vec<NodeValidation>,
    pub diagnostics: Vec<FederationDiagnostic>,
    pub divergence: Option<String>,
}

pub fn fixture_catalog() -> Vec<FederationFixture> {
    vec![
        fixture("2-node federation", 2, TopologyKind::Linear),
        fixture("3-node federation", 3, TopologyKind::Star),
        fixture("5-node federation", 5, TopologyKind::Mesh),
        fixture("partitioned federation", 5, TopologyKind::Partitioned),
        fixture("restored federation", 3, TopologyKind::Recovered),
        fixture("archive recovery federation", 3, TopologyKind::Recovered),
    ]
}

fn fixture(id: &str, nodes: usize, topology: TopologyKind) -> FederationFixture {
    FederationFixture {
        federation_id: id.to_string(),
        node_count: nodes,
        topology,
        replay_lineage: format!("lineage:{id}"),
        workload: vec!["spawn".into(), "tick".into(), "commit".into()],
    }
}

pub fn run_fixture(
    fixture: &FederationFixture,
    divergence: Option<DivergenceType>,
) -> FederationReport {
    let mut validations = Vec::new();
    let mut diagnostics = Vec::new();
    let mut canonical_roots: Option<(String, String, String)> = None;
    let mut divergence_msg = None;

    for idx in 0..fixture.node_count {
        let node_id = format!("node-{:02}", idx + 1);
        let mut ordered_workload = fixture.workload.clone();
        if matches!(divergence, Some(DivergenceType::OutOfOrderReplay))
            && idx == fixture.node_count - 1
        {
            ordered_workload.reverse();
        }

        let replay_root = digest(&[
            fixture.federation_id.as_str(),
            &fixture.replay_lineage,
            &ordered_workload.join("|"),
        ]);
        let mut receipt_root = digest(&[&replay_root, "receipt"]);
        let mut state_root = digest(&[&replay_root, "state"]);
        let mut archive_root = digest(&[&replay_root, &receipt_root, &state_root, "archive"]);

        if idx == fixture.node_count - 1 {
            match divergence {
                Some(DivergenceType::ReceiptMismatch) => {
                    receipt_root = digest(&[&receipt_root, "diverge"])
                }
                Some(DivergenceType::StateDiffMismatch) => {
                    state_root = digest(&[&state_root, "diverge"])
                }
                Some(DivergenceType::ReplayTruncation) => {
                    let truncated = fixture
                        .workload
                        .iter()
                        .take(fixture.workload.len().saturating_sub(1))
                        .cloned()
                        .collect::<Vec<_>>();
                    let truncated_root =
                        digest(&[fixture.federation_id.as_str(), &truncated.join("|")]);
                    archive_root =
                        digest(&[&truncated_root, &receipt_root, &state_root, "archive"]);
                }
                Some(DivergenceType::ArchiveCorruption) => {
                    archive_root = digest(&[&archive_root, "corrupt"])
                }
                Some(DivergenceType::TopologyMismatch) => {
                    archive_root = digest(&[&archive_root, "topology-mismatch"])
                }
                Some(DivergenceType::InvalidRestoration) => {
                    state_root = digest(&[&state_root, "invalid-restore"])
                }
                Some(DivergenceType::OutOfOrderReplay) | None => {}
            }
        }

        let equivalent = if let Some((r, rc, s)) = &canonical_roots {
            r == &replay_root && rc == &receipt_root && s == &state_root
        } else {
            canonical_roots = Some((
                replay_root.clone(),
                receipt_root.clone(),
                state_root.clone(),
            ));
            true
        };

        if !equivalent && divergence_msg.is_none() {
            divergence_msg = Some(format!(
                "divergence at {node_id}: replay/receipt/state root mismatch"
            ));
        }

        validations.push(NodeValidation {
            node_id: node_id.clone(),
            replay_root: replay_root.clone(),
            receipt_root: receipt_root.clone(),
            state_root: state_root.clone(),
            equivalent,
        });
        diagnostics.push(FederationDiagnostic {
            federation_id: fixture.federation_id.clone(),
            node_id,
            topology_role: role_for(fixture.topology, idx),
            replay_root,
            archive_root,
            receipt_root,
            divergence_detected: !equivalent,
            recovery_possible: equivalent
                || !matches!(divergence, Some(DivergenceType::InvalidRestoration)),
        });
    }

    FederationReport {
        validations,
        diagnostics,
        divergence: divergence_msg,
    }
}

fn digest(parts: &[&str]) -> String {
    let mut h = Sha256::new();
    for part in parts {
        h.update(part.as_bytes());
        h.update([0u8]);
    }
    hex::encode(h.finalize())
}

fn role_for(topology: TopologyKind, idx: usize) -> String {
    match topology {
        TopologyKind::Linear => if idx == 0 { "head" } else { "chain" }.to_string(),
        TopologyKind::Star => if idx == 0 { "hub" } else { "spoke" }.to_string(),
        TopologyKind::Mesh => "peer".to_string(),
        TopologyKind::Partitioned => if idx % 2 == 0 {
            "partition-a"
        } else {
            "partition-b"
        }
        .to_string(),
        TopologyKind::Recovered => if idx == 0 { "restorer" } else { "rejoined" }.to_string(),
    }
}

pub fn topology_manifest(fixture: &FederationFixture) -> BTreeMap<String, BTreeSet<String>> {
    let mut map = BTreeMap::new();
    for idx in 0..fixture.node_count {
        let node = format!("node-{:02}", idx + 1);
        let peers = match fixture.topology {
            TopologyKind::Linear => [
                idx.checked_sub(1),
                if idx + 1 < fixture.node_count {
                    Some(idx + 1)
                } else {
                    None
                },
            ]
            .iter()
            .flatten()
            .map(|p| format!("node-{:02}", p + 1))
            .collect(),
            TopologyKind::Star => {
                if idx == 0 {
                    (1..fixture.node_count)
                        .map(|p| format!("node-{:02}", p + 1))
                        .collect()
                } else {
                    ["node-01".to_string()].into_iter().collect()
                }
            }
            TopologyKind::Mesh => (0..fixture.node_count)
                .filter(|p| *p != idx)
                .map(|p| format!("node-{:02}", p + 1))
                .collect(),
            TopologyKind::Partitioned => (0..fixture.node_count)
                .filter(|p| *p != idx && p % 2 == idx % 2)
                .map(|p| format!("node-{:02}", p + 1))
                .collect(),
            TopologyKind::Recovered => (0..fixture.node_count)
                .filter(|p| *p != idx)
                .map(|p| format!("node-{:02}", p + 1))
                .collect(),
        };
        map.insert(node, peers);
    }
    map
}
