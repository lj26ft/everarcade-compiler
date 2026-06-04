//! GPU Runtime v0.1 scaffold.
//!
//! The operational validation path for v0.1 is the deterministic shell model in
//! `gpu/jobs/gpu_model.sh`. This Rust module mirrors the public data contract so
//! future renderer clients can bind to the same non-authoritative concepts
//! without introducing live GPU inspection or protocol-state mutation.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuJob {
    pub job_id: String,
    pub projection_root: String,
    pub job_type: GpuJobType,
    pub priority: u32,
    pub submission_epoch: u64,
    pub job_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuJobType {
    WorldRender,
    EntityRender,
    PhysicsVisualization,
    InventoryVisualization,
    EventVisualization,
    ReplayRender,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuDeviceCapability {
    pub gpu_identifier: String,
    pub memory_mb: u64,
    pub compute_capability: String,
    pub queue_capacity: u32,
    pub runtime_version: String,
    pub device_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuWorker {
    pub worker_id: String,
    pub device_id: String,
    pub capability_profile: String,
    pub capacity: u32,
    pub availability: String,
    pub worker_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderArtifact {
    pub projection_root: String,
    pub worker_root: String,
    pub job_root: String,
    pub artifact_root: String,
    pub render_root: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RendererGpuIntegration {
    pub projection_export_root: String,
    pub gpu_job_submission_root: String,
    pub artifact_import_root: String,
    pub integration_root: String,
}
