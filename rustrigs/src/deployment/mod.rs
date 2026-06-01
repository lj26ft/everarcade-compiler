use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, DeploymentIntentRecord, ProtocolRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentIntentInput {
    pub package: String,
    pub target: String,
    pub version: String,
    pub tick: u64,
}
fn rec(action: &str, i: &DeploymentIntentInput) -> DeploymentIntentRecord {
    DeploymentIntentRecord::new(
        action,
        i.package.clone(),
        fields(&[
            ("target", i.target.clone()),
            ("version", i.version.clone()),
            ("tick", i.tick.to_string()),
            ("execution", String::from("runtime-orchestrator-only")),
        ]),
    )
}
pub struct CreateDeploymentIntent;
impl Rustrig for CreateDeploymentIntent {
    type Input = DeploymentIntentInput;
    type Output = DeploymentIntentRecord;
    fn execute(i: Self::Input) -> Self::Output {
        rec("create-deployment-intent", &i)
    }
}
impl ReplaySafeRustrig for CreateDeploymentIntent {}
impl VersionedRustrig for CreateDeploymentIntent {
    const NAME: &'static str = "CreateDeploymentIntent";
    const VERSION: &'static str = "1.0.0";
    const RECORD_TYPE: &'static str = "DeploymentIntentRecord";
}
pub fn create_deployment_intent(i: DeploymentIntentInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::DeploymentIntent(rec(
        "create-deployment-intent",
        &i,
    ))]
}
pub fn create_upgrade_intent(i: DeploymentIntentInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::DeploymentIntent(rec(
        "create-upgrade-intent",
        &i,
    ))]
}
pub fn create_recovery_intent(i: DeploymentIntentInput) -> Vec<ProtocolRecord> {
    vec![ProtocolRecord::DeploymentIntent(rec(
        "create-recovery-intent",
        &i,
    ))]
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    [
        "CreateDeploymentIntent",
        "CreateUpgradeIntent",
        "CreateRecoveryIntent",
    ]
    .into_iter()
    .map(|n| RustrigDescriptor::new(n, "1.0.0", "DeploymentIntentRecord"))
    .collect()
}
