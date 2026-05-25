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
