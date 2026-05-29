use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatorDiagnostic {
    pub surface: &'static str,
    pub deterministic: bool,
    pub replay_continuity: &'static str,
    pub authority_boundary: &'static str,
    pub renderer_authoritative: bool,
    pub mutation_policy: &'static str,
    pub lineage_hash: String,
}

pub fn stable_hash(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part.as_bytes());
    }
    hex::encode(hasher.finalize())
}

pub fn diagnostic(surface: &'static str, lineage: &[&str]) -> CreatorDiagnostic {
    CreatorDiagnostic {
        surface,
        deterministic: true,
        replay_continuity: "preserved",
        authority_boundary: "deterministic-execution-runtime-only",
        renderer_authoritative: false,
        mutation_policy: "replay-safe-read-model-and-validated-append-only-packages",
        lineage_hash: stable_hash(lineage),
    }
}

pub fn reject_authority_bypass(requested: bool) -> Result<(), &'static str> {
    if requested {
        Err("creator tooling cannot bypass deterministic runtime authority")
    } else {
        Ok(())
    }
}

pub fn reject_replay_mutation(requested: bool) -> Result<(), &'static str> {
    if requested {
        Err("creator tooling cannot mutate replay lineage")
    } else {
        Ok(())
    }
}

pub mod editor {
    pub mod runtime {
        include!("../editor/src/runtime.rs");
    }
    pub mod world {
        include!("../editor/src/world.rs");
    }
    pub mod session {
        include!("../editor/src/session.rs");
    }
    pub mod viewport {
        include!("../editor/src/viewport.rs");
    }
    pub mod replay {
        include!("../editor/src/replay.rs");
    }
    pub mod validation {
        include!("../editor/src/validation.rs");
    }
}

pub mod ecs_editor {
    pub mod entity {
        include!("../ecs-editor/src/entity.rs");
    }
    pub mod component {
        include!("../ecs-editor/src/component.rs");
    }
    pub mod system {
        include!("../ecs-editor/src/system.rs");
    }
    pub mod archetype {
        include!("../ecs-editor/src/archetype.rs");
    }
    pub mod validation {
        include!("../ecs-editor/src/validation.rs");
    }
}

pub mod replay_visualizer {
    pub mod timeline {
        include!("../replay-visualizer/src/timeline.rs");
    }
    pub mod window {
        include!("../replay-visualizer/src/window.rs");
    }
    pub mod checkpoint {
        include!("../replay-visualizer/src/checkpoint.rs");
    }
    pub mod divergence {
        include!("../replay-visualizer/src/divergence.rs");
    }
    pub mod session {
        include!("../replay-visualizer/src/session.rs");
    }
}

pub mod asset_pipeline {
    pub mod import {
        include!("../asset-pipeline/src/import.rs");
    }
    pub mod manifest {
        include!("../asset-pipeline/src/manifest.rs");
    }
    pub mod conversion {
        include!("../asset-pipeline/src/conversion.rs");
    }
    pub mod validation {
        include!("../asset-pipeline/src/validation.rs");
    }
    pub mod hash {
        include!("../asset-pipeline/src/hash.rs");
    }
}

pub mod hot_reload {
    pub mod runtime {
        include!("../hot-reload/src/runtime.rs");
    }
    pub mod assets {
        include!("../hot-reload/src/assets.rs");
    }
    pub mod state {
        include!("../hot-reload/src/state.rs");
    }
    pub mod recovery {
        include!("../hot-reload/src/recovery.rs");
    }
    pub mod validation {
        include!("../hot-reload/src/validation.rs");
    }
}

pub mod entity_inspector {
    pub mod entity {
        include!("../entity-inspector/src/entity.rs");
    }
    pub mod component {
        include!("../entity-inspector/src/component.rs");
    }
    pub mod runtime {
        include!("../entity-inspector/src/runtime.rs");
    }
    pub mod session {
        include!("../entity-inspector/src/session.rs");
    }
    pub mod validation {
        include!("../entity-inspector/src/validation.rs");
    }
}

pub mod simulation_debugger {
    pub mod runtime {
        include!("../simulation-debugger/src/runtime.rs");
    }
    pub mod ai {
        include!("../simulation-debugger/src/ai.rs");
    }
    pub mod scheduler {
        include!("../simulation-debugger/src/scheduler.rs");
    }
    pub mod partition {
        include!("../simulation-debugger/src/partition.rs");
    }
    pub mod validation {
        include!("../simulation-debugger/src/validation.rs");
    }
}

pub mod creator_dashboard {
    pub mod projects {
        include!("../creator-dashboard/src/projects.rs");
    }
    pub mod deployment {
        include!("../creator-dashboard/src/deployment.rs");
    }
    pub mod replay {
        include!("../creator-dashboard/src/replay.rs");
    }
    pub mod packages {
        include!("../creator-dashboard/src/packages.rs");
    }
    pub mod validation {
        include!("../creator-dashboard/src/validation.rs");
    }
}

pub mod content_packager {
    pub mod package {
        include!("../content-packager/src/package.rs");
    }
    pub mod archive {
        include!("../content-packager/src/archive.rs");
    }
    pub mod signing {
        include!("../content-packager/src/signing.rs");
    }
    pub mod runtime {
        include!("../content-packager/src/runtime.rs");
    }
    pub mod validation {
        include!("../content-packager/src/validation.rs");
    }
}
