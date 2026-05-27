use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeSurfaceStatus {
    Production,
    ActiveIntegration,
    Scaffold,
    Deprecated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSurfaceClassification {
    pub module: &'static str,
    pub status: RuntimeSurfaceStatus,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSurfaceAuditResult {
    pub classifications: Vec<RuntimeSurfaceClassification>,
    pub export_inconsistencies: Vec<&'static str>,
    pub scaffold_modules: Vec<&'static str>,
    pub deprecated_modules: Vec<&'static str>,
}

pub struct RuntimeSurfaceAudit;

impl RuntimeSurfaceAudit {
    pub fn run() -> RuntimeSurfaceAuditResult {
        let classifications = vec![
            RuntimeSurfaceClassification {
                module: "execution_core::runtime::ci",
                status: RuntimeSurfaceStatus::ActiveIntegration,
                reason: "certification/staging runtime validation",
            },
            RuntimeSurfaceClassification {
                module: "execution_core::runtime::validation",
                status: RuntimeSurfaceStatus::Production,
                reason: "deterministic validation runtime",
            },
            RuntimeSurfaceClassification {
                module: "renderer_client::history",
                status: RuntimeSurfaceStatus::Scaffold,
                reason: "non-authoritative replay/history scaffolding",
            },
            RuntimeSurfaceClassification {
                module: "renderer_client::federation",
                status: RuntimeSurfaceStatus::Scaffold,
                reason: "future renderer federation integration",
            },
            RuntimeSurfaceClassification {
                module: "renderer_client::transport_runtime",
                status: RuntimeSurfaceStatus::Scaffold,
                reason: "future renderer transport integration",
            },
        ];

        RuntimeSurfaceAuditResult {
            export_inconsistencies: vec![],
            scaffold_modules: classifications
                .iter()
                .filter(|c| c.status == RuntimeSurfaceStatus::Scaffold)
                .map(|c| c.module)
                .collect(),
            deprecated_modules: classifications
                .iter()
                .filter(|c| c.status == RuntimeSurfaceStatus::Deprecated)
                .map(|c| c.module)
                .collect(),
            classifications,
        }
    }

    pub fn status_counts() -> BTreeMap<&'static str, usize> {
        let mut counts = BTreeMap::new();
        for c in Self::run().classifications {
            let k = match c.status {
                RuntimeSurfaceStatus::Production => "Production",
                RuntimeSurfaceStatus::ActiveIntegration => "ActiveIntegration",
                RuntimeSurfaceStatus::Scaffold => "Scaffold",
                RuntimeSurfaceStatus::Deprecated => "Deprecated",
            };
            *counts.entry(k).or_insert(0) += 1;
        }
        counts
    }
}
