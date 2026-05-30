use crate::{ReplaySafeRustrig, Rustrig, RustrigDescriptor, VersionedRustrig};
use contract_api::protocol_records::{fields, DeploymentIntentRecord};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentIntentInput {
    pub package: String,
    pub target: String,
    pub version: String,
    pub tick: u64,
}
pub struct CreateDeploymentIntent;
impl Rustrig for CreateDeploymentIntent {
    type Input = DeploymentIntentInput;
    type Output = DeploymentIntentRecord;
    fn execute(i: Self::Input) -> Self::Output {
        DeploymentIntentRecord::new(
            "create-deployment-intent",
            i.package,
            fields(&[
                ("target", i.target),
                ("version", i.version),
                ("tick", i.tick.to_string()),
                ("execution", String::from("runtime-orchestrator-only")),
            ]),
        )
    }
}
impl ReplaySafeRustrig for CreateDeploymentIntent {}
impl VersionedRustrig for CreateDeploymentIntent {
    const NAME: &'static str = "CreateDeploymentIntent";
    const VERSION: &'static str = "1.0.0";
    const RECORD_TYPE: &'static str = "DeploymentIntentRecord";
}
pub fn descriptors() -> Vec<RustrigDescriptor> {
    vec![RustrigDescriptor::new(
        "CreateDeploymentIntent",
        "1.0.0",
        "DeploymentIntentRecord",
    )]
}
