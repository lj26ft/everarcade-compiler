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
