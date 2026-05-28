use serde::{Deserialize, Serialize};

use sha2::{Digest, Sha256};

fn deterministic_root(parts: &[&str]) -> String {
    let mut bytes = Vec::new();
    for part in parts {
        bytes.extend_from_slice(&(part.len() as u64).to_le_bytes());
        bytes.extend_from_slice(part.as_bytes());
    }
    let digest = Sha256::digest(&bytes);
    hex::encode(digest)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReplaySession {
    pub session_id: String,
    pub checkpoint_index: u64,
    pub replay_root: String,
    pub continuity_root: String,
    pub interrupted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReplayCheckpoint {
    pub session_id: String,
    pub checkpoint_index: u64,
    pub replay_root: String,
    pub continuity_root: String,
    pub checkpoint_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeSessionLifecycleRuntime;

impl RuntimeSessionLifecycleRuntime {
    pub fn create_replay_session(session_id: &str, replay_root: &str) -> RuntimeReplaySession {
        let continuity_root = deterministic_root(&["session", session_id, replay_root, "0"]);
        RuntimeReplaySession {
            session_id: session_id.to_string(),
            checkpoint_index: 0,
            replay_root: replay_root.to_string(),
            continuity_root,
            interrupted: false,
        }
    }

    pub fn checkpoint_session(session: &RuntimeReplaySession) -> RuntimeReplayCheckpoint {
        RuntimeReplayCheckpoint {
            session_id: session.session_id.clone(),
            checkpoint_index: session.checkpoint_index + 1,
            replay_root: session.replay_root.clone(),
            continuity_root: session.continuity_root.clone(),
            checkpoint_root: deterministic_root(&[
                "checkpoint",
                &session.session_id,
                &session.replay_root,
                &session.continuity_root,
                &(session.checkpoint_index + 1).to_string(),
            ]),
        }
    }

    pub fn restore_session(
        checkpoint: &RuntimeReplayCheckpoint,
    ) -> Result<RuntimeReplaySession, String> {
        let expected_root = deterministic_root(&[
            "checkpoint",
            &checkpoint.session_id,
            &checkpoint.replay_root,
            &checkpoint.continuity_root,
            &checkpoint.checkpoint_index.to_string(),
        ]);
        if checkpoint.checkpoint_root != expected_root {
            return Err("checkpoint corruption detected".into());
        }
        Ok(RuntimeReplaySession {
            session_id: checkpoint.session_id.clone(),
            checkpoint_index: checkpoint.checkpoint_index,
            replay_root: checkpoint.replay_root.clone(),
            continuity_root: checkpoint.continuity_root.clone(),
            interrupted: false,
        })
    }

    pub fn recover_interrupted_session(
        checkpoint: &RuntimeReplayCheckpoint,
    ) -> Result<RuntimeReplaySession, String> {
        Self::restore_session(checkpoint).map(|mut session| {
            session.interrupted = false;
            session
        })
    }

    pub fn validate_replay_equivalence(
        left: &RuntimeReplaySession,
        right: &RuntimeReplaySession,
    ) -> Result<(), String> {
        if left.session_id == right.session_id
            && left.replay_root == right.replay_root
            && left.continuity_root == right.continuity_root
            && left.checkpoint_index == right.checkpoint_index
        {
            Ok(())
        } else {
            Err("restored replay session diverged".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDaemonCheckpoint {
    pub node_id: String,
    pub replay_height: u64,
    pub continuity_root: String,
    pub checkpoint_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDaemonState {
    pub node_id: String,
    pub replay_height: u64,
    pub continuity_root: String,
    pub running: bool,
    pub healthy: bool,
    pub authoritative: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeDaemonRecoveryRuntime;

impl RuntimeDaemonRecoveryRuntime {
    pub fn start(node_id: &str) -> RuntimeDaemonState {
        RuntimeDaemonState {
            node_id: node_id.to_string(),
            replay_height: 0,
            continuity_root: deterministic_root(&["daemon", node_id, "genesis"]),
            running: true,
            healthy: true,
            authoritative: false,
        }
    }

    pub fn checkpoint(state: &RuntimeDaemonState) -> RuntimeDaemonCheckpoint {
        RuntimeDaemonCheckpoint {
            node_id: state.node_id.clone(),
            replay_height: state.replay_height,
            continuity_root: state.continuity_root.clone(),
            checkpoint_root: deterministic_root(&[
                "daemon-checkpoint",
                &state.node_id,
                &state.replay_height.to_string(),
                &state.continuity_root,
            ]),
        }
    }

    pub fn recover_from_checkpoint(
        checkpoint: &RuntimeDaemonCheckpoint,
    ) -> Result<RuntimeDaemonState, String> {
        let expected = deterministic_root(&[
            "daemon-checkpoint",
            &checkpoint.node_id,
            &checkpoint.replay_height.to_string(),
            &checkpoint.continuity_root,
        ]);
        if checkpoint.checkpoint_root != expected {
            return Err("daemon checkpoint corruption detected".into());
        }
        Ok(RuntimeDaemonState {
            node_id: checkpoint.node_id.clone(),
            replay_height: checkpoint.replay_height,
            continuity_root: checkpoint.continuity_root.clone(),
            running: true,
            healthy: true,
            authoritative: false,
        })
    }

    pub fn restart(state: &RuntimeDaemonState) -> Result<RuntimeDaemonState, String> {
        let checkpoint = Self::checkpoint(state);
        Self::recover_from_checkpoint(&checkpoint)
    }

    pub fn readiness(state: &RuntimeDaemonState) -> Result<(), String> {
        if state.running && state.healthy && !state.authoritative {
            Ok(())
        } else {
            Err("daemon readiness gate failed".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPersistenceCheckpoint {
    pub checkpoint_id: String,
    pub frame_index: u64,
    pub replay_root: String,
    pub continuity_root: String,
    pub checkpoint_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplayPersistenceRuntime;

impl ReplayPersistenceRuntime {
    pub fn persist_checkpoint(
        checkpoint_id: &str,
        frame_index: u64,
        replay_root: &str,
        continuity_root: &str,
    ) -> ReplayPersistenceCheckpoint {
        ReplayPersistenceCheckpoint {
            checkpoint_id: checkpoint_id.to_string(),
            frame_index,
            replay_root: replay_root.to_string(),
            continuity_root: continuity_root.to_string(),
            checkpoint_root: deterministic_root(&[
                "renderer-persistence",
                checkpoint_id,
                &frame_index.to_string(),
                replay_root,
                continuity_root,
            ]),
        }
    }

    pub fn restore_checkpoint(
        checkpoint: &ReplayPersistenceCheckpoint,
    ) -> Result<ReplayPersistenceCheckpoint, String> {
        let expected = deterministic_root(&[
            "renderer-persistence",
            &checkpoint.checkpoint_id,
            &checkpoint.frame_index.to_string(),
            &checkpoint.replay_root,
            &checkpoint.continuity_root,
        ]);
        if checkpoint.checkpoint_root != expected {
            return Err("replay checkpoint corruption detected".into());
        }
        Ok(checkpoint.clone())
    }

    pub fn restore_continuity_root(
        checkpoint: &ReplayPersistenceCheckpoint,
    ) -> Result<String, String> {
        Self::restore_checkpoint(checkpoint).map(|restored| restored.continuity_root)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeServiceCycle {
    pub cycle_index: u64,
    pub replay_root: String,
    pub continuity_root: String,
    pub cycle_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeNodeServiceState {
    pub node_id: String,
    pub cycles: Vec<RuntimeServiceCycle>,
    pub checkpoint_root: String,
    pub running: bool,
    pub reconstruction_only: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeNodeServiceRuntime;

impl RuntimeNodeServiceRuntime {
    pub fn start(node_id: &str, replay_root: &str) -> RuntimeNodeServiceState {
        RuntimeNodeServiceState {
            node_id: node_id.to_string(),
            cycles: vec![Self::cycle(0, replay_root, replay_root)],
            checkpoint_root: deterministic_root(&["service-checkpoint", node_id, replay_root, "0"]),
            running: true,
            reconstruction_only: true,
        }
    }

    fn cycle(cycle_index: u64, replay_root: &str, previous_root: &str) -> RuntimeServiceCycle {
        let cycle = cycle_index.to_string();
        let continuity_root =
            deterministic_root(&["service-cycle", &cycle, replay_root, previous_root]);
        RuntimeServiceCycle {
            cycle_index,
            replay_root: replay_root.to_string(),
            continuity_root: continuity_root.clone(),
            cycle_root: deterministic_root(&[
                "service-cycle-root",
                &cycle,
                replay_root,
                &continuity_root,
            ]),
        }
    }

    pub fn process_cycles(state: &mut RuntimeNodeServiceState, cycles: u64) {
        for _ in 0..cycles {
            let last = state
                .cycles
                .last()
                .expect("runtime service always has genesis cycle");
            let next = Self::cycle(
                last.cycle_index + 1,
                &last.replay_root,
                &last.continuity_root,
            );
            state.checkpoint_root = deterministic_root(&[
                "service-checkpoint",
                &state.node_id,
                &next.continuity_root,
                &next.cycle_index.to_string(),
            ]);
            state.cycles.push(next);
        }
    }

    pub fn checkpoint(state: &RuntimeNodeServiceState) -> RuntimeDaemonCheckpoint {
        let last = state
            .cycles
            .last()
            .expect("runtime service checkpoint requires cycle");
        RuntimeDaemonCheckpoint {
            node_id: state.node_id.clone(),
            replay_height: last.cycle_index,
            continuity_root: last.continuity_root.clone(),
            checkpoint_root: deterministic_root(&[
                "runtime-service-checkpoint",
                &state.node_id,
                &last.cycle_index.to_string(),
                &last.continuity_root,
            ]),
        }
    }

    pub fn restore(
        checkpoint: &RuntimeDaemonCheckpoint,
    ) -> Result<RuntimeNodeServiceState, String> {
        let expected = deterministic_root(&[
            "runtime-service-checkpoint",
            &checkpoint.node_id,
            &checkpoint.replay_height.to_string(),
            &checkpoint.continuity_root,
        ]);
        if checkpoint.checkpoint_root != expected {
            return Err("runtime service checkpoint corruption detected".into());
        }
        Ok(RuntimeNodeServiceState {
            node_id: checkpoint.node_id.clone(),
            cycles: vec![RuntimeServiceCycle {
                cycle_index: checkpoint.replay_height,
                replay_root: checkpoint.continuity_root.clone(),
                continuity_root: checkpoint.continuity_root.clone(),
                cycle_root: deterministic_root(&[
                    "restored-service-cycle",
                    &checkpoint.node_id,
                    &checkpoint.replay_height.to_string(),
                    &checkpoint.continuity_root,
                ]),
            }],
            checkpoint_root: checkpoint.checkpoint_root.clone(),
            running: true,
            reconstruction_only: true,
        })
    }

    pub fn equivalent(left: &RuntimeNodeServiceState, right: &RuntimeNodeServiceState) -> bool {
        left.cycles
            .last()
            .map(|c| (&c.cycle_index, &c.continuity_root))
            == right
                .cycles
                .last()
                .map(|c| (&c.cycle_index, &c.continuity_root))
            && left.reconstruction_only
            && right.reconstruction_only
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplayStorageEngineRuntime {
    pub checkpoints: Vec<ReplayPersistenceCheckpoint>,
}

impl ReplayStorageEngineRuntime {
    pub fn append(&mut self, checkpoint: ReplayPersistenceCheckpoint) -> Result<(), String> {
        ReplayPersistenceRuntime::restore_checkpoint(&checkpoint)?;
        if self
            .checkpoints
            .iter()
            .any(|c| c.checkpoint_id == checkpoint.checkpoint_id)
        {
            return Err("duplicate replay checkpoint rejected".into());
        }
        self.checkpoints.push(checkpoint);
        Ok(())
    }

    pub fn restore_latest(&self) -> Result<ReplayPersistenceCheckpoint, String> {
        let checkpoint = self
            .checkpoints
            .last()
            .ok_or_else(|| "no replay checkpoint available".to_string())?;
        ReplayPersistenceRuntime::restore_checkpoint(checkpoint)
    }

    pub fn compact(&self) -> Result<ReplayPersistenceCheckpoint, String> {
        self.restore_latest()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportReplayChunk {
    pub sequence: u64,
    pub payload_root: String,
    pub continuity_root: String,
    pub chunk_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveTransportBoundaryRuntime {
    pub chunks: Vec<TransportReplayChunk>,
}

impl LiveTransportBoundaryRuntime {
    pub fn chunk(sequence: u64, payload_root: &str, previous_root: &str) -> TransportReplayChunk {
        let seq = sequence.to_string();
        let continuity_root =
            deterministic_root(&["transport-continuity", &seq, payload_root, previous_root]);
        TransportReplayChunk {
            sequence,
            payload_root: payload_root.to_string(),
            continuity_root: continuity_root.clone(),
            chunk_root: deterministic_root(&[
                "transport-chunk",
                &seq,
                payload_root,
                &continuity_root,
            ]),
        }
    }

    pub fn ingest(&mut self, chunk: TransportReplayChunk) -> Result<(), String> {
        let seq = chunk.sequence.to_string();
        let expected = deterministic_root(&[
            "transport-chunk",
            &seq,
            &chunk.payload_root,
            &chunk.continuity_root,
        ]);
        if chunk.chunk_root != expected {
            return Err("transport corruption rejected".into());
        }
        if self.chunks.iter().any(|c| c.sequence == chunk.sequence) {
            return Err("transport duplication rejected".into());
        }
        if chunk.sequence != self.chunks.len() as u64 {
            return Err("transport divergence rejected".into());
        }
        self.chunks.push(chunk);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerReplaySessionState {
    pub peer_id: String,
    pub window_start: u64,
    pub window_end: u64,
    pub continuity_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PeerSessionRuntime;

impl PeerSessionRuntime {
    pub fn establish(peer_id: &str, continuity_root: &str) -> PeerReplaySessionState {
        PeerReplaySessionState {
            peer_id: peer_id.to_string(),
            window_start: 0,
            window_end: 0,
            continuity_root: continuity_root.to_string(),
        }
    }

    pub fn synchronize_window(session: &mut PeerReplaySessionState, end: u64) {
        session.window_end = end;
        session.continuity_root = deterministic_root(&[
            "peer-window",
            &session.peer_id,
            &session.window_start.to_string(),
            &end.to_string(),
            &session.continuity_root,
        ]);
    }

    pub fn recover(session: &PeerReplaySessionState) -> PeerReplaySessionState {
        session.clone()
    }

    pub fn validate(
        left: &PeerReplaySessionState,
        right: &PeerReplaySessionState,
    ) -> Result<(), String> {
        if left == right {
            Ok(())
        } else {
            Err("peer replay divergence rejected".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObserverStreamState {
    pub observer_id: String,
    pub observed_windows: u64,
    pub continuity_root: String,
    pub non_authoritative: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ObserverStreamRuntime;

impl ObserverStreamRuntime {
    pub fn stream(observer_id: &str, windows: u64, continuity_root: &str) -> ObserverStreamState {
        ObserverStreamState {
            observer_id: observer_id.to_string(),
            observed_windows: windows,
            continuity_root: deterministic_root(&[
                "observer-stream",
                observer_id,
                &windows.to_string(),
                continuity_root,
            ]),
            non_authoritative: true,
        }
    }

    pub fn restore(state: &ObserverStreamState) -> Result<ObserverStreamState, String> {
        if state.non_authoritative {
            Ok(state.clone())
        } else {
            Err("authoritative observer stream rejected".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReadinessState {
    pub runtime_ready: bool,
    pub replay_recovery_ready: bool,
    pub peer_synchronized: bool,
    pub observer_synchronized: bool,
    pub continuity_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeServiceHealthRuntime;

impl RuntimeServiceHealthRuntime {
    pub fn evaluate(continuity_root: &str) -> RuntimeReadinessState {
        RuntimeReadinessState {
            runtime_ready: true,
            replay_recovery_ready: true,
            peer_synchronized: true,
            observer_synchronized: true,
            continuity_root: continuity_root.to_string(),
        }
    }

    pub fn gate(state: &RuntimeReadinessState) -> Result<(), String> {
        if state.runtime_ready
            && state.replay_recovery_ready
            && state.peer_synchronized
            && state.observer_synchronized
            && !state.continuity_root.is_empty()
        {
            Ok(())
        } else {
            Err("runtime service health gate rejected invalid continuity".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeDeploymentState {
    pub deployment_id: String,
    pub topology_root: String,
    pub continuity_root: String,
    pub restoration_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeDeploymentRestorationRuntime;

impl RuntimeDeploymentRestorationRuntime {
    pub fn capture(
        deployment_id: &str,
        topology_root: &str,
        continuity_root: &str,
    ) -> RuntimeDeploymentState {
        RuntimeDeploymentState {
            deployment_id: deployment_id.to_string(),
            topology_root: topology_root.to_string(),
            continuity_root: continuity_root.to_string(),
            restoration_root: deterministic_root(&[
                "deployment-restoration",
                deployment_id,
                topology_root,
                continuity_root,
            ]),
        }
    }

    pub fn restore(state: &RuntimeDeploymentState) -> Result<RuntimeDeploymentState, String> {
        let expected = deterministic_root(&[
            "deployment-restoration",
            &state.deployment_id,
            &state.topology_root,
            &state.continuity_root,
        ]);
        if state.restoration_root == expected {
            Ok(state.clone())
        } else {
            Err("deployment restoration corruption rejected".into())
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeAdversarialValidation;

impl RuntimeAdversarialValidation {
    pub fn reject_replay_corruption(mut checkpoint: ReplayPersistenceCheckpoint) -> bool {
        checkpoint.replay_root.push_str("::corrupt");
        ReplayPersistenceRuntime::restore_checkpoint(&checkpoint).is_err()
    }

    pub fn reject_checkpoint_corruption(mut checkpoint: RuntimeDaemonCheckpoint) -> bool {
        checkpoint.checkpoint_root.push_str("::corrupt");
        RuntimeNodeServiceRuntime::restore(&checkpoint).is_err()
    }

    pub fn reject_transport_duplication() -> bool {
        let mut boundary = LiveTransportBoundaryRuntime::default();
        let chunk = LiveTransportBoundaryRuntime::chunk(0, "payload", "root");
        boundary.ingest(chunk.clone()).is_ok() && boundary.ingest(chunk).is_err()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationReplayFrame {
    pub sequence: u64,
    pub replay_root: String,
    pub continuity_root: String,
    pub frame_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationCheckpoint {
    pub checkpoint_id: String,
    pub sequence: u64,
    pub topology_root: String,
    pub continuity_root: String,
    pub checkpoint_root: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimePeerFederationState {
    pub peer_id: String,
    pub topology_root: String,
    pub replay_window_start: u64,
    pub replay_window_end: u64,
    pub continuity_root: String,
    pub session_root: String,
    pub reconstruction_only: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeFederationRuntime;

impl RuntimeFederationRuntime {
    pub fn handshake(
        peer_id: &str,
        topology_root: &str,
        continuity_root: &str,
    ) -> RuntimePeerFederationState {
        RuntimePeerFederationState {
            peer_id: peer_id.to_string(),
            topology_root: topology_root.to_string(),
            replay_window_start: 0,
            replay_window_end: 0,
            continuity_root: continuity_root.to_string(),
            session_root: deterministic_root(&[
                "federation-peer-session",
                peer_id,
                topology_root,
                continuity_root,
            ]),
            reconstruction_only: true,
        }
    }

    pub fn synchronize_window(
        state: &mut RuntimePeerFederationState,
        window_end: u64,
    ) -> Result<(), String> {
        if window_end < state.replay_window_end {
            return Err("replay truncation rejected".into());
        }
        state.replay_window_start = state.replay_window_end;
        state.replay_window_end = window_end;
        state.continuity_root = deterministic_root(&[
            "federation-window",
            &state.peer_id,
            &state.replay_window_start.to_string(),
            &state.replay_window_end.to_string(),
            &state.topology_root,
            &state.continuity_root,
        ]);
        state.session_root = deterministic_root(&[
            "federation-peer-session",
            &state.peer_id,
            &state.topology_root,
            &state.continuity_root,
        ]);
        Ok(())
    }

    pub fn reconnect(
        state: &RuntimePeerFederationState,
    ) -> Result<RuntimePeerFederationState, String> {
        Self::validate_peer_lineage(state)?;
        Ok(state.clone())
    }

    pub fn validate_peer_lineage(state: &RuntimePeerFederationState) -> Result<(), String> {
        let expected = deterministic_root(&[
            "federation-peer-session",
            &state.peer_id,
            &state.topology_root,
            &state.continuity_root,
        ]);
        if state.reconstruction_only && state.session_root == expected {
            Ok(())
        } else {
            Err("invalid peer lineage rejected".into())
        }
    }

    pub fn validate_equivalence(
        left: &RuntimePeerFederationState,
        right: &RuntimePeerFederationState,
    ) -> Result<(), String> {
        if left == right && left.reconstruction_only && right.reconstruction_only {
            Ok(())
        } else {
            Err("federation replay divergence rejected".into())
        }
    }

    pub fn checkpoint(
        state: &RuntimePeerFederationState,
        checkpoint_id: &str,
    ) -> FederationCheckpoint {
        FederationCheckpoint {
            checkpoint_id: checkpoint_id.to_string(),
            sequence: state.replay_window_end,
            topology_root: state.topology_root.clone(),
            continuity_root: state.continuity_root.clone(),
            checkpoint_root: deterministic_root(&[
                "federation-checkpoint",
                checkpoint_id,
                &state.replay_window_end.to_string(),
                &state.topology_root,
                &state.continuity_root,
            ]),
        }
    }

    pub fn restore_checkpoint(
        checkpoint: &FederationCheckpoint,
    ) -> Result<FederationCheckpoint, String> {
        let expected = deterministic_root(&[
            "federation-checkpoint",
            &checkpoint.checkpoint_id,
            &checkpoint.sequence.to_string(),
            &checkpoint.topology_root,
            &checkpoint.continuity_root,
        ]);
        if checkpoint.checkpoint_root == expected {
            Ok(checkpoint.clone())
        } else {
            Err("corrupted federation checkpoint rejected".into())
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DistributedReplayTransportRuntime {
    pub frames: Vec<FederationReplayFrame>,
}

impl DistributedReplayTransportRuntime {
    pub fn frame(
        sequence: u64,
        replay_root: &str,
        previous_continuity_root: &str,
    ) -> FederationReplayFrame {
        let seq = sequence.to_string();
        let continuity_root = deterministic_root(&[
            "distributed-frame-continuity",
            &seq,
            replay_root,
            previous_continuity_root,
        ]);
        FederationReplayFrame {
            sequence,
            replay_root: replay_root.to_string(),
            continuity_root: continuity_root.clone(),
            frame_root: deterministic_root(&[
                "distributed-frame",
                &seq,
                replay_root,
                &continuity_root,
            ]),
        }
    }

    pub fn propagate(&mut self, frame: FederationReplayFrame) -> Result<(), String> {
        let expected = deterministic_root(&[
            "distributed-frame",
            &frame.sequence.to_string(),
            &frame.replay_root,
            &frame.continuity_root,
        ]);
        if frame.frame_root != expected {
            return Err("replay corruption rejected".into());
        }
        if self
            .frames
            .iter()
            .any(|existing| existing.sequence == frame.sequence)
        {
            return Err("replay duplication rejected".into());
        }
        if frame.sequence != self.frames.len() as u64 {
            return Err("replay truncation or divergence rejected".into());
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn recover(&self) -> Result<Self, String> {
        let mut recovered = Self::default();
        for frame in &self.frames {
            recovered.propagate(frame.clone())?;
        }
        Ok(recovered)
    }

    pub fn continuity_root(&self) -> Option<&str> {
        self.frames
            .last()
            .map(|frame| frame.continuity_root.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationRouteEnvelope {
    pub route_kind: String,
    pub source_peer: String,
    pub target_peer: String,
    pub sequence: u64,
    pub continuity_root: String,
    pub route_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FederationRoutingRuntime {
    pub routes: Vec<FederationRouteEnvelope>,
}

impl FederationRoutingRuntime {
    pub fn route(
        kind: &str,
        source_peer: &str,
        target_peer: &str,
        sequence: u64,
        continuity_root: &str,
    ) -> FederationRouteEnvelope {
        FederationRouteEnvelope {
            route_kind: kind.to_string(),
            source_peer: source_peer.to_string(),
            target_peer: target_peer.to_string(),
            sequence,
            continuity_root: continuity_root.to_string(),
            route_root: deterministic_root(&[
                "federation-route",
                kind,
                source_peer,
                target_peer,
                &sequence.to_string(),
                continuity_root,
            ]),
        }
    }

    pub fn accept(&mut self, envelope: FederationRouteEnvelope) -> Result<(), String> {
        let expected = deterministic_root(&[
            "federation-route",
            &envelope.route_kind,
            &envelope.source_peer,
            &envelope.target_peer,
            &envelope.sequence.to_string(),
            &envelope.continuity_root,
        ]);
        if envelope.route_root != expected {
            return Err("invalid replay routing lineage rejected".into());
        }
        if self.routes.iter().any(|route| {
            route.sequence == envelope.sequence && route.route_kind == envelope.route_kind
        }) {
            return Err("duplicate replay route rejected".into());
        }
        if envelope.sequence != self.routes.len() as u64 {
            return Err("replay routing order divergence rejected".into());
        }
        self.routes.push(envelope);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationTopologyState {
    pub peers: Vec<String>,
    pub continuity_root: String,
    pub topology_root: String,
    pub reconstruction_only: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FederationTopologyRuntime;

impl FederationTopologyRuntime {
    pub fn topology(peers: &[&str], continuity_root: &str) -> FederationTopologyState {
        let mut sorted: Vec<String> = peers.iter().map(|peer| peer.to_string()).collect();
        sorted.sort();
        let peer_joined = sorted.join("|");
        FederationTopologyState {
            peers: sorted,
            continuity_root: continuity_root.to_string(),
            topology_root: deterministic_root(&[
                "federation-topology",
                &peer_joined,
                continuity_root,
            ]),
            reconstruction_only: true,
        }
    }

    pub fn recover(state: &FederationTopologyState) -> Result<FederationTopologyState, String> {
        let peer_joined = state.peers.join("|");
        let expected =
            deterministic_root(&["federation-topology", &peer_joined, &state.continuity_root]);
        if state.topology_root == expected && state.reconstruction_only {
            Ok(state.clone())
        } else {
            Err("invalid federation restoration rejected".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayStreamPropagationState {
    pub stream_id: String,
    pub propagated_windows: u64,
    pub checkpoint_root: String,
    pub continuity_root: String,
    pub reconstruction_only: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplayStreamPropagationRuntime;

impl ReplayStreamPropagationRuntime {
    pub fn propagate(
        stream_id: &str,
        windows: u64,
        previous_continuity_root: &str,
    ) -> ReplayStreamPropagationState {
        let continuity_root = deterministic_root(&[
            "stream-propagation",
            stream_id,
            &windows.to_string(),
            previous_continuity_root,
        ]);
        ReplayStreamPropagationState {
            stream_id: stream_id.to_string(),
            propagated_windows: windows,
            checkpoint_root: deterministic_root(&[
                "stream-checkpoint",
                stream_id,
                &windows.to_string(),
                &continuity_root,
            ]),
            continuity_root,
            reconstruction_only: true,
        }
    }

    pub fn restore(
        state: &ReplayStreamPropagationState,
    ) -> Result<ReplayStreamPropagationState, String> {
        let expected = deterministic_root(&[
            "stream-checkpoint",
            &state.stream_id,
            &state.propagated_windows.to_string(),
            &state.continuity_root,
        ]);
        if state.checkpoint_root == expected && state.reconstruction_only {
            Ok(state.clone())
        } else {
            Err("corrupted stream propagation rejected".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FederationHealthState {
    pub peer_synchronized: bool,
    pub transport_continuity: bool,
    pub topology_recovered: bool,
    pub stream_recovered: bool,
    pub continuity_root: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuntimeFederationHealthRuntime;

impl RuntimeFederationHealthRuntime {
    pub fn evaluate(continuity_root: &str) -> FederationHealthState {
        FederationHealthState {
            peer_synchronized: true,
            transport_continuity: true,
            topology_recovered: true,
            stream_recovered: true,
            continuity_root: continuity_root.to_string(),
        }
    }

    pub fn gate(state: &FederationHealthState) -> Result<(), String> {
        if state.peer_synchronized
            && state.transport_continuity
            && state.topology_recovered
            && state.stream_recovered
            && !state.continuity_root.is_empty()
        {
            Ok(())
        } else {
            Err("runtime federation readiness rejected".into())
        }
    }
}
