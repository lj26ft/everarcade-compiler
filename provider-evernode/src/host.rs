use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostBootstrapState {
    pub host_id: String,
    pub runtime_installed: bool,
    pub runtime_validated: bool,
    pub package_deployed: bool,
    pub health_verified: bool,
    pub operator_registered: bool,
    pub events: Vec<String>,
}

impl HostBootstrapState {
    pub fn new(host_id: impl Into<String>) -> Self {
        Self {
            host_id: host_id.into(),
            ..Self::default()
        }
    }
    pub fn bootstrap(&mut self) {
        self.runtime_installed = true;
        self.runtime_validated = true;
        self.package_deployed = true;
        self.health_verified = true;
        self.operator_registered = true;
        for event in [
            "fresh host bootstrap",
            "runtime install",
            "runtime validation",
            "package deployment",
            "health verification",
            "operator registration",
        ] {
            if !self.events.iter().any(|e| e == event) {
                self.events.push(event.into());
            }
        }
    }
    pub fn is_ready(&self) -> bool {
        self.runtime_installed
            && self.runtime_validated
            && self.package_deployed
            && self.health_verified
            && self.operator_registered
    }
}
