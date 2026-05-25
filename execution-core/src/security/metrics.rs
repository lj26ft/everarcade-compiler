#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityMetrics {
    pub capability_violations: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IsolationMetrics {
    pub isolation_violations: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResourceMetrics {
    pub resource_overflows: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AbuseMetrics {
    pub replay_abuse_attempts: u64,
    pub event_amplification_attempts: u64,
    pub witness_overflow_attempts: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QuarantineMetrics {
    pub quarantine_events: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MutationGovernanceMetrics {
    pub accepted_operations: u64,
    pub rejected_operations: u64,
    pub overflow_count: u64,
    pub quarantine_count: u64,
    pub rejection_roots: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EventGovernanceMetrics {
    pub accepted_operations: u64,
    pub rejected_operations: u64,
    pub overflow_count: u64,
    pub quarantine_count: u64,
    pub rejection_roots: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WitnessGovernanceMetrics {
    pub accepted_operations: u64,
    pub rejected_operations: u64,
    pub overflow_count: u64,
    pub quarantine_count: u64,
    pub rejection_roots: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ReplayGovernanceMetrics {
    pub accepted_operations: u64,
    pub rejected_operations: u64,
    pub overflow_count: u64,
    pub quarantine_count: u64,
    pub rejection_roots: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SnapshotGovernanceMetrics {
    pub accepted_operations: u64,
    pub rejected_operations: u64,
    pub overflow_count: u64,
    pub quarantine_count: u64,
    pub rejection_roots: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IsolationEnforcementMetrics {
    pub accepted_operations: u64,
    pub rejected_operations: u64,
    pub isolation_violations: u64,
    pub quarantine_count: u64,
    pub rejection_roots: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CapabilityEnforcementMetrics {
    pub accepted_operations: u64,
    pub rejected_operations: u64,
    pub capability_violations: u64,
    pub quarantine_count: u64,
    pub rejection_roots: Vec<String>,
}
