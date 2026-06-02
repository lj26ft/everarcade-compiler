use control_plane::leases::LeaseResources;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EverNodeLeaseState {
    Pending,
    Allocated,
    Running,
    Recovering,
    Expired,
    Migrating,
    Released,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EverNodeLease {
    pub lease_id: String,
    pub host_id: String,
    pub game_id: String,
    pub state: EverNodeLeaseState,
    pub resources: LeaseResources,
    pub audit: Vec<String>,
}

impl EverNodeLease {
    pub fn allocate(
        id: impl Into<String>,
        host: impl Into<String>,
        game: impl Into<String>,
        resources: LeaseResources,
    ) -> Self {
        Self {
            lease_id: id.into(),
            host_id: host.into(),
            game_id: game.into(),
            state: EverNodeLeaseState::Allocated,
            resources,
            audit: vec!["lease discovery".into(), "lease allocation".into()],
        }
    }
    pub fn renew(&mut self) {
        self.audit.push("lease renewal".into());
    }
    pub fn expire(&mut self) {
        self.state = EverNodeLeaseState::Expired;
        self.audit.push("lease expiration".into());
    }
    pub fn recover(&mut self) {
        self.state = EverNodeLeaseState::Recovering;
        self.audit.push("lease recovery".into());
    }
    pub fn migrate(&mut self, host: impl Into<String>) {
        self.state = EverNodeLeaseState::Migrating;
        self.host_id = host.into();
        self.audit.push("lease migration".into());
    }
    pub fn release(&mut self) {
        self.state = EverNodeLeaseState::Released;
        self.audit.push("lease release".into());
    }
    pub fn mark_running(&mut self) {
        self.state = EverNodeLeaseState::Running;
        self.audit.push("lease running".into());
    }
    pub fn audit(&self) -> Vec<String> {
        self.audit.clone()
    }
}
