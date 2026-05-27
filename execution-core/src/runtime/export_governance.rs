use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum RuntimeExportVisibility {
    PublicStable,
    InternalIntegration,
    Scaffold,
    Deprecated,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeExportSurface {
    pub category: &'static str,
    pub export_path: &'static str,
    pub visibility: RuntimeExportVisibility,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeExportOwnership {
    pub owner_module: &'static str,
    pub export: RuntimeExportSurface,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimePublicApi {
    pub canonical_path: &'static str,
    pub visibility: RuntimeExportVisibility,
    pub domain: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeSymbolLineage {
    pub symbol: &'static str,
    pub owner_module: &'static str,
    pub canonical_path: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeApiOwnership {
    pub symbol: &'static str,
    pub owner_module: &'static str,
    pub visibility: RuntimeExportVisibility,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RuntimeApiContinuityAudit {
    pub unresolved_symbols: Vec<&'static str>,
    pub invalid_alias_exports: Vec<&'static str>,
    pub duplicate_owners: Vec<&'static str>,
    pub disconnected_public_apis: Vec<&'static str>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RuntimeNamespaceAudit {
    pub unresolved_symbols: Vec<&'static str>,
    pub wildcard_export_modules: Vec<&'static str>,
    pub stale_exports: Vec<&'static str>,
    pub duplicate_owners: Vec<&'static str>,
    pub disconnected_surfaces: Vec<&'static str>,
}

pub fn runtime_export_ownership() -> Vec<RuntimeExportOwnership> {
    vec![
        RuntimeExportOwnership {
            owner_module: "runtime::history",
            export: RuntimeExportSurface {
                category: "replay/history runtime",
                export_path:
                    "runtime::renderer_client::history::timeline::HistoricalReplayTimeline",
                visibility: RuntimeExportVisibility::PublicStable,
            },
        },
        RuntimeExportOwnership {
            owner_module: "runtime::transport",
            export: RuntimeExportSurface {
                category: "replay transport runtime",
                export_path:
                    "runtime::renderer_client::transport_runtime::stream::ReplayTransportStream",
                visibility: RuntimeExportVisibility::InternalIntegration,
            },
        },
    ]
}

pub fn runtime_public_api() -> Vec<RuntimePublicApi> {
    vec![
        RuntimePublicApi {
            canonical_path: "execution_core::runtime::validation::runtime::ValidationDagRuntime",
            visibility: RuntimeExportVisibility::PublicStable,
            domain: "validation runtime",
        },
        RuntimePublicApi {
            canonical_path: "execution_core::runtime::ci::runtime::CiExecutionHistoryRuntime",
            visibility: RuntimeExportVisibility::InternalIntegration,
            domain: "ci orchestration runtime",
        },
    ]
}

pub fn runtime_symbol_lineage() -> Vec<RuntimeSymbolLineage> {
    vec![
        RuntimeSymbolLineage {
            symbol: "ValidationDagRuntime",
            owner_module: "execution_core::runtime::validation::runtime",
            canonical_path: "execution_core::runtime::validation::runtime::ValidationDagRuntime",
        },
        RuntimeSymbolLineage {
            symbol: "CiExecutionHistoryRuntime",
            owner_module: "execution_core::runtime::ci::runtime",
            canonical_path: "execution_core::runtime::ci::runtime::CiExecutionHistoryRuntime",
        },
    ]
}

pub fn runtime_api_ownership() -> Vec<RuntimeApiOwnership> {
    runtime_symbol_lineage()
        .into_iter()
        .map(|lineage| RuntimeApiOwnership {
            symbol: lineage.symbol,
            owner_module: lineage.owner_module,
            visibility: RuntimeExportVisibility::PublicStable,
        })
        .collect()
}

pub fn runtime_api_continuity_audit() -> RuntimeApiContinuityAudit {
    RuntimeApiContinuityAudit::default()
}
