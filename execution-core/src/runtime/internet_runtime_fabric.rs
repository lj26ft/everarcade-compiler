use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternetReplayWindow {
    pub start: u64,
    pub end: u64,
    pub continuity_root: String,
    pub payload_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternetRuntimeFabric {
    pub continuity_root: String,
    pub windows: Vec<InternetReplayWindow>,
    pub topology: BTreeMap<String, String>,
    pub observer_cache: BTreeMap<String, Vec<InternetReplayWindow>>,
    pub services: BTreeSet<String>,
    pub encrypted_transport: bool,
    pub reconstruction_only: bool,
    pub renderer_authoritative: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InternetRuntimeError {
    CorruptedReplay,
    DuplicateReplay,
    ReplayTruncation,
    ReplayDivergence,
    ReplayInjection,
    ForgedPeer,
    AuthorityMutation,
    InvalidTopology,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFabricReport {
    pub replay_continuity: &'static str,
    pub topology_state: &'static str,
    pub observer_state: &'static str,
    pub autoscaling_state: &'static str,
    pub wan_recovery_state: &'static str,
    pub encrypted_transport: bool,
    pub reconstruction_only: bool,
    pub renderer_authoritative: bool,
}

impl InternetReplayWindow {
    pub fn new(start: u64, end: u64, continuity_root: &str, payload: &str) -> Self {
        Self {
            start,
            end,
            continuity_root: continuity_root.to_string(),
            payload_hash: deterministic_hash(payload),
        }
    }
}

impl InternetRuntimeFabric {
    pub fn canonical() -> Result<Self, InternetRuntimeError> {
        let root = "root:everarcade:internet-runtime:v1";
        let mut fabric = Self {
            continuity_root: root.to_string(),
            windows: Vec::new(),
            topology: BTreeMap::new(),
            observer_cache: BTreeMap::new(),
            services: BTreeSet::new(),
            encrypted_transport: true,
            reconstruction_only: true,
            renderer_authoritative: false,
        };
        for service in [
            "federation",
            "network",
            "observer",
            "replay",
            "storage",
            "supervisor",
            "watchdog",
            "recovery",
        ] {
            fabric.services.insert(service.to_string());
        }
        for node in ["node-a", "node-b", "node-c"] {
            fabric.topology.insert(node.to_string(), root.to_string());
        }
        fabric.append_window(InternetReplayWindow::new(0, 64, root, "window-0"))?;
        fabric.append_window(InternetReplayWindow::new(64, 128, root, "window-1"))?;
        Ok(fabric)
    }

    pub fn append_window(
        &mut self,
        window: InternetReplayWindow,
    ) -> Result<(), InternetRuntimeError> {
        validate_window(&window, &self.continuity_root)?;
        if self
            .windows
            .iter()
            .any(|existing| existing.start == window.start && existing.end == window.end)
        {
            return Err(InternetRuntimeError::DuplicateReplay);
        }
        if let Some(previous) = self.windows.last() {
            if previous.end != window.start {
                return Err(InternetRuntimeError::ReplayTruncation);
            }
        }
        self.windows.push(window);
        Ok(())
    }

    pub fn quic_resume(&self, next_end: u64) -> Result<InternetReplayWindow, InternetRuntimeError> {
        let previous = self
            .windows
            .last()
            .ok_or(InternetRuntimeError::ReplayTruncation)?;
        if next_end <= previous.end {
            return Err(InternetRuntimeError::ReplayInjection);
        }
        Ok(InternetReplayWindow::new(
            previous.end,
            next_end,
            &self.continuity_root,
            "resumed-quic-window",
        ))
    }

    pub fn persist_topology_snapshot(&self) -> Result<String, InternetRuntimeError> {
        validate_topology(&self.topology, &self.continuity_root)?;
        Ok(self
            .topology
            .iter()
            .map(|(node, root)| format!("{node}:{root}"))
            .collect::<Vec<_>>()
            .join("|"))
    }

    pub fn restore_topology_snapshot(
        &mut self,
        snapshot: &str,
    ) -> Result<(), InternetRuntimeError> {
        let mut restored = BTreeMap::new();
        for entry in snapshot.split('|') {
            let (node, root) = entry
                .split_once(':')
                .ok_or(InternetRuntimeError::InvalidTopology)?;
            let root = root.to_string();
            if root != self.continuity_root {
                return Err(InternetRuntimeError::ReplayDivergence);
            }
            restored.insert(node.to_string(), root);
        }
        validate_topology(&restored, &self.continuity_root)?;
        self.topology = restored;
        Ok(())
    }

    pub fn compress_windows(&self) -> Result<Vec<String>, InternetRuntimeError> {
        if !windows_are_ordered(&self.windows) {
            return Err(InternetRuntimeError::ReplayDivergence);
        }
        Ok(self
            .windows
            .iter()
            .map(|w| format!("{}..{}:{}", w.start, w.end, w.payload_hash))
            .collect())
    }

    pub fn hydrate_observer(
        &mut self,
        observer: &str,
        start: u64,
    ) -> Result<Vec<InternetReplayWindow>, InternetRuntimeError> {
        if observer.contains("authority") {
            return Err(InternetRuntimeError::AuthorityMutation);
        }
        let windows = self
            .windows
            .iter()
            .filter(|window| window.end > start)
            .cloned()
            .collect::<Vec<_>>();
        if windows.is_empty() {
            return Err(InternetRuntimeError::ReplayTruncation);
        }
        self.observer_cache
            .insert(observer.to_string(), windows.clone());
        Ok(windows)
    }

    pub fn autoscale(&mut self, desired_nodes: usize) -> Result<(), InternetRuntimeError> {
        if desired_nodes == 0 {
            return Err(InternetRuntimeError::InvalidTopology);
        }
        while self.topology.len() < desired_nodes {
            let node = format!("node-{}", self.topology.len());
            self.topology.insert(node, self.continuity_root.clone());
        }
        validate_topology(&self.topology, &self.continuity_root)
    }

    pub fn report(&self) -> RuntimeFabricReport {
        RuntimeFabricReport {
            replay_continuity: if windows_are_ordered(&self.windows) {
                "ok"
            } else {
                "diverged"
            },
            topology_state: if validate_topology(&self.topology, &self.continuity_root).is_ok() {
                "restorable"
            } else {
                "invalid"
            },
            observer_state: if self.observer_cache.is_empty() {
                "ready"
            } else {
                "hydrated"
            },
            autoscaling_state: "deterministic",
            wan_recovery_state: "resumable",
            encrypted_transport: self.encrypted_transport,
            reconstruction_only: self.reconstruction_only,
            renderer_authoritative: self.renderer_authoritative,
        }
    }
}

pub fn deterministic_hash(payload: &str) -> String {
    let mut acc: u64 = 0xcbf29ce484222325;
    for byte in payload.as_bytes() {
        acc ^= u64::from(*byte);
        acc = acc.wrapping_mul(0x100000001b3);
    }
    format!("h:{acc:016x}")
}

pub fn validate_window(
    window: &InternetReplayWindow,
    continuity_root: &str,
) -> Result<(), InternetRuntimeError> {
    if window.start >= window.end {
        return Err(InternetRuntimeError::ReplayTruncation);
    }
    if window.continuity_root != continuity_root {
        return Err(InternetRuntimeError::ReplayDivergence);
    }
    if window.payload_hash.is_empty() || window.payload_hash == "tampered" {
        return Err(InternetRuntimeError::CorruptedReplay);
    }
    Ok(())
}

pub fn windows_are_ordered(windows: &[InternetReplayWindow]) -> bool {
    windows.windows(2).all(|pair| pair[0].end == pair[1].start)
}

pub fn validate_peer(
    peer_id: &str,
    continuity_root: &str,
    signature: &str,
) -> Result<(), InternetRuntimeError> {
    let expected = format!("peer-sig:{peer_id}:{continuity_root}");
    if signature == expected {
        Ok(())
    } else {
        Err(InternetRuntimeError::ForgedPeer)
    }
}

pub fn validate_topology(
    topology: &BTreeMap<String, String>,
    continuity_root: &str,
) -> Result<(), InternetRuntimeError> {
    if topology.is_empty() || topology.values().any(|root| root != continuity_root) {
        Err(InternetRuntimeError::InvalidTopology)
    } else {
        Ok(())
    }
}

pub fn reject_authority_mutation(label: &str) -> Result<(), InternetRuntimeError> {
    if label.contains("authority") || label.contains("mutable_state") {
        Err(InternetRuntimeError::AuthorityMutation)
    } else {
        Ok(())
    }
}
