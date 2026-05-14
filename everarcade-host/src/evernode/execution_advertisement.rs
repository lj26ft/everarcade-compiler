use super::capacity_manifest::ExecutionCapacityManifest;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionAdvertisement {
    pub manifest: ExecutionCapacityManifest,
    pub available: bool,
}
