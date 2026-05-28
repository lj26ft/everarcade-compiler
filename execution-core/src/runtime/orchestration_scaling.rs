use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayWindow {
    pub start: u64,
    pub end: u64,
    pub continuity_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeNode {
    pub id: String,
    pub continuity_root: String,
    pub online: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayPeer {
    pub id: String,
    pub lineage_owner: String,
    pub signature: String,
    pub trusted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShardRoute {
    pub shard_id: u64,
    pub node_id: String,
    pub window: ReplayWindow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeOrchestrationReport {
    pub continuity_root: String,
    pub node_count: usize,
    pub peer_count: usize,
    pub shard_count: usize,
    pub replay_only: bool,
    pub renderer_authoritative: bool,
    pub ordering_preserved: bool,
    pub recovery_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeFederationError {
    DuplicateReplayWindow,
    ReplayTruncation,
    ReplayCorruption,
    ReplayDivergence,
    ReplayInjection,
    UnauthorizedPeer,
    InvalidTopology,
    AuthorityMutation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFederationOrchestrator {
    continuity_root: String,
    nodes: BTreeMap<String, RuntimeNode>,
    peers: BTreeMap<String, ReplayPeer>,
    windows: Vec<ReplayWindow>,
}

impl RuntimeFederationOrchestrator {
    pub fn new(continuity_root: impl Into<String>) -> Self {
        Self {
            continuity_root: continuity_root.into(),
            nodes: BTreeMap::new(),
            peers: BTreeMap::new(),
            windows: Vec::new(),
        }
    }

    pub fn register_node(&mut self, id: impl Into<String>) -> Result<(), RuntimeFederationError> {
        let id = id.into();
        if id.is_empty() || self.nodes.contains_key(&id) {
            return Err(RuntimeFederationError::InvalidTopology);
        }
        self.nodes.insert(
            id.clone(),
            RuntimeNode {
                id,
                continuity_root: self.continuity_root.clone(),
                online: true,
            },
        );
        Ok(())
    }

    pub fn register_peer(&mut self, peer: ReplayPeer) -> Result<(), RuntimeFederationError> {
        if !validate_peer_signature(&peer, &self.continuity_root) {
            return Err(RuntimeFederationError::UnauthorizedPeer);
        }
        self.peers.insert(peer.id.clone(), peer);
        Ok(())
    }

    pub fn append_window(&mut self, window: ReplayWindow) -> Result<(), RuntimeFederationError> {
        validate_window(&window, &self.continuity_root)?;
        if let Some(last) = self.windows.last() {
            if window.start != last.end {
                return Err(RuntimeFederationError::ReplayTruncation);
            }
        }
        if self
            .windows
            .iter()
            .any(|w| w.start == window.start && w.end == window.end)
        {
            return Err(RuntimeFederationError::DuplicateReplayWindow);
        }
        self.windows.push(window);
        Ok(())
    }

    pub fn route_shards(
        &self,
        shard_count: u64,
    ) -> Result<Vec<ShardRoute>, RuntimeFederationError> {
        if shard_count == 0 || self.nodes.is_empty() || self.windows.is_empty() {
            return Err(RuntimeFederationError::InvalidTopology);
        }
        let node_ids: Vec<_> = self.nodes.keys().cloned().collect();
        let latest = self.windows.last().expect("checked non-empty").clone();
        Ok((0..shard_count)
            .map(|shard_id| ShardRoute {
                shard_id,
                node_id: node_ids[shard_id as usize % node_ids.len()].clone(),
                window: latest.clone(),
            })
            .collect())
    }

    pub fn restore_topology(&mut self) -> Result<(), RuntimeFederationError> {
        if self.nodes.is_empty() {
            return Err(RuntimeFederationError::InvalidTopology);
        }
        for node in self.nodes.values_mut() {
            node.online = true;
            node.continuity_root = self.continuity_root.clone();
        }
        Ok(())
    }

    pub fn report(&self, shard_count: usize) -> RuntimeOrchestrationReport {
        RuntimeOrchestrationReport {
            continuity_root: self.continuity_root.clone(),
            node_count: self.nodes.len(),
            peer_count: self.peers.len(),
            shard_count,
            replay_only: true,
            renderer_authoritative: false,
            ordering_preserved: windows_are_ordered(&self.windows),
            recovery_ready: !self.nodes.is_empty(),
        }
    }
}

pub fn deterministic_signature(
    peer_id: &str,
    lineage_owner: &str,
    continuity_root: &str,
) -> String {
    format!("sig:{peer_id}:{lineage_owner}:{continuity_root}")
}

pub fn trusted_peer(id: &str, owner: &str, continuity_root: &str) -> ReplayPeer {
    ReplayPeer {
        id: id.to_string(),
        lineage_owner: owner.to_string(),
        signature: deterministic_signature(id, owner, continuity_root),
        trusted: true,
    }
}

pub fn validate_peer_signature(peer: &ReplayPeer, continuity_root: &str) -> bool {
    peer.trusted
        && peer.signature == deterministic_signature(&peer.id, &peer.lineage_owner, continuity_root)
}

pub fn validate_window(
    window: &ReplayWindow,
    continuity_root: &str,
) -> Result<(), RuntimeFederationError> {
    if window.start >= window.end {
        return Err(RuntimeFederationError::ReplayTruncation);
    }
    if window.continuity_root != continuity_root {
        return Err(RuntimeFederationError::ReplayDivergence);
    }
    Ok(())
}

pub fn windows_are_ordered(windows: &[ReplayWindow]) -> bool {
    windows.windows(2).all(|pair| pair[0].end == pair[1].start)
}

pub fn throttle_capacity(inflight: u64, capacity: u64) -> Result<u64, RuntimeFederationError> {
    if inflight > capacity {
        return Err(RuntimeFederationError::ReplayCorruption);
    }
    Ok(capacity - inflight)
}

pub fn recover_stalled_stream(
    windows: &[ReplayWindow],
) -> Result<ReplayWindow, RuntimeFederationError> {
    if !windows_are_ordered(windows) || windows.is_empty() {
        return Err(RuntimeFederationError::ReplayDivergence);
    }
    Ok(windows.last().expect("checked non-empty").clone())
}

pub fn quic_resume_window(
    previous: &ReplayWindow,
    next_end: u64,
) -> Result<ReplayWindow, RuntimeFederationError> {
    if next_end <= previous.end {
        return Err(RuntimeFederationError::ReplayInjection);
    }
    Ok(ReplayWindow {
        start: previous.end,
        end: next_end,
        continuity_root: previous.continuity_root.clone(),
    })
}

pub fn hydrate_observers(
    observer_count: usize,
    windows: &[ReplayWindow],
) -> Result<BTreeMap<usize, Vec<ReplayWindow>>, RuntimeFederationError> {
    if observer_count == 0 || !windows_are_ordered(windows) {
        return Err(RuntimeFederationError::ReplayDivergence);
    }
    let mut partitions: BTreeMap<usize, Vec<ReplayWindow>> = BTreeMap::new();
    for (idx, window) in windows.iter().cloned().enumerate() {
        partitions
            .entry(idx % observer_count)
            .or_default()
            .push(window);
    }
    Ok(partitions)
}

pub fn deployment_manifest(
    nodes: &[RuntimeNode],
    continuity_root: &str,
) -> Result<String, RuntimeFederationError> {
    if nodes.is_empty() || nodes.iter().any(|n| n.continuity_root != continuity_root) {
        return Err(RuntimeFederationError::InvalidTopology);
    }
    let ids: Vec<_> = nodes.iter().map(|n| n.id.as_str()).collect();
    Ok(format!(
        "continuity_root={continuity_root}\nnodes={}\n",
        ids.join(",")
    ))
}

pub fn package_bundle_root(parts: &[&str]) -> Result<String, RuntimeFederationError> {
    if parts
        .iter()
        .any(|p| p.is_empty() || p.contains("mutable_authority"))
    {
        return Err(RuntimeFederationError::AuthorityMutation);
    }
    let unique: BTreeSet<_> = parts.iter().copied().collect();
    Ok(format!(
        "evernode-bundle:{}",
        unique.into_iter().collect::<Vec<_>>().join("+")
    ))
}

pub fn canonical_fixture() -> Result<RuntimeFederationOrchestrator, RuntimeFederationError> {
    let root = "root:everarcade:federation:v1";
    let mut orchestrator = RuntimeFederationOrchestrator::new(root);
    orchestrator.register_node("node-a")?;
    orchestrator.register_node("node-b")?;
    orchestrator.register_node("node-c")?;
    orchestrator.register_peer(trusted_peer("peer-a", "lineage-a", root))?;
    orchestrator.register_peer(trusted_peer("peer-b", "lineage-b", root))?;
    orchestrator.append_window(ReplayWindow {
        start: 0,
        end: 64,
        continuity_root: root.to_string(),
    })?;
    orchestrator.append_window(ReplayWindow {
        start: 64,
        end: 128,
        continuity_root: root.to_string(),
    })?;
    Ok(orchestrator)
}
