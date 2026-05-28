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
