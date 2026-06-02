use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const CANONICAL_STATE_DIRS: [&str; 7] = [
    "world",
    "replay",
    "checkpoints",
    "receipts",
    "packages",
    "anchors",
    "operator",
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotPocketRuntimeConfig {
    pub contract_bundle: String,
    pub runtime_package: String,
    pub state_root: String,
    pub config_path: String,
    pub startup_command: Vec<String>,
}

impl HotPocketRuntimeConfig {
    pub fn arena_vanguard(state_root: impl Into<String>) -> Self {
        Self {
            contract_bundle: "arena-vanguard.hotpocket.bundle".into(),
            runtime_package: "everarcade-hotpocket-runtime".into(),
            state_root: state_root.into(),
            config_path: "state/operator/hotpocket.json".into(),
            startup_command: vec![
                "hotpocket".into(),
                "contract".into(),
                "run".into(),
                "--conf".into(),
                "state/operator/hotpocket.json".into(),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotPocketStateLayout {
    pub root: String,
    pub authority_dirs: Vec<String>,
}

impl HotPocketStateLayout {
    pub fn canonical(root: impl Into<String>) -> Self {
        Self {
            root: root.into(),
            authority_dirs: CANONICAL_STATE_DIRS.iter().map(|d| d.to_string()).collect(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        let expected: BTreeSet<_> = CANONICAL_STATE_DIRS.iter().copied().collect();
        let actual: BTreeSet<_> = self.authority_dirs.iter().map(String::as_str).collect();
        if actual != expected {
            return Err(
                "hotpocket state layout contains unknown or missing authority folders".into(),
            );
        }
        Ok(())
    }

    pub fn validate_authority_write(&self, path: &str) -> Result<(), String> {
        let normalized = path.trim_start_matches("./");
        if !normalized.starts_with("state/") {
            return Err("authority write outside state/ rejected".into());
        }
        let mut parts = normalized.split('/');
        let _state = parts.next();
        let authority = parts
            .next()
            .ok_or_else(|| "missing authority folder".to_string())?;
        if CANONICAL_STATE_DIRS.contains(&authority) {
            Ok(())
        } else {
            Err("unknown authority folder rejected".into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeLifecycle {
    pub runtime_id: String,
    pub config: HotPocketRuntimeConfig,
    pub process_state: String,
    pub events: Vec<String>,
}

impl RuntimeLifecycle {
    pub fn new(runtime_id: impl Into<String>, config: HotPocketRuntimeConfig) -> Self {
        Self {
            runtime_id: runtime_id.into(),
            config,
            process_state: "installed".into(),
            events: vec!["install hotpocket runtime".into()],
        }
    }
    pub fn start(&mut self) {
        self.process_state = "running".into();
        self.events.push("start hotpocket".into());
    }
    pub fn stop(&mut self) {
        self.process_state = "stopped".into();
        self.events.push("shutdown hotpocket".into());
    }
    pub fn restart(&mut self) {
        self.process_state = "running".into();
        self.events.push("restart hotpocket".into());
    }
}
