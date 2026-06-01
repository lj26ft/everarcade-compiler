use crate::alerts::{AlertLevel, AlertManager, AlertTrigger};
use crate::cost_model::{arena_vanguard_cost_model, ArenaVanguardCostModel};
use crate::deployment::DeploymentOrchestrator;
use crate::leases::LeaseManager;
use crate::logs::{LogKind, LogStore};
use crate::metrics::MetricsSnapshot;
use crate::topology::FederationTopology;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnchorKind {
    Receipt,
    Replay,
    Checkpoint,
    Deployment,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorPayload {
    pub kind: AnchorKind,
    pub payload_hash: String,
    pub external_settlement_required: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorIntent {
    pub payload: AnchorPayload,
    pub settlement_service: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorControlPlane {
    pub orchestrator: DeploymentOrchestrator,
    pub alerts: AlertManager,
    pub logs: LogStore,
    pub topology: FederationTopology,
}
impl OperatorControlPlane {
    pub fn arena_vanguard() -> Self {
        Self {
            orchestrator: DeploymentOrchestrator::new(LeaseManager::arena_vanguard()),
            alerts: AlertManager::default(),
            logs: LogStore::default(),
            topology: FederationTopology::new(5).expect("supported topology"),
        }
    }
    pub fn lease_audit(&self) -> Vec<String> {
        self.orchestrator.leases.audit_lease_usage()
    }
    pub fn metrics(&self) -> MetricsSnapshot {
        self.orchestrator.metrics.snapshot.clone()
    }
    pub fn cost_model(&self) -> ArenaVanguardCostModel {
        arena_vanguard_cost_model()
    }
    pub fn generate_anchor_payload(
        &self,
        kind: AnchorKind,
        payload_hash: impl Into<String>,
    ) -> AnchorPayload {
        AnchorPayload {
            kind,
            payload_hash: payload_hash.into(),
            external_settlement_required: true,
        }
    }
    pub fn generate_anchor_intent(
        &self,
        kind: AnchorKind,
        payload_hash: impl Into<String>,
    ) -> AnchorIntent {
        AnchorIntent {
            payload: self.generate_anchor_payload(kind, payload_hash),
            settlement_service: "external-xrpl-settlement".into(),
        }
    }
    pub fn record_operator_action(&mut self, action: &str) {
        self.logs.append(
            0,
            LogKind::OperatorAction,
            action,
            vec![("actor".into(), "operator".into())],
        );
    }
    pub fn emit_lease_exhaustion(&mut self) {
        self.alerts.emit(
            AlertLevel::Critical,
            AlertTrigger::LeaseExhaustion,
            "lease-manager",
            "lease exhaustion",
        );
    }
}
