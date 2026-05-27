#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeExportVisibility {
    PublicStable,
    InternalIntegration,
    Scaffold,
    Deprecated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeExportSurface {
    pub category: &'static str,
    pub export_path: &'static str,
    pub visibility: RuntimeExportVisibility,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeExportOwnership {
    pub owner_module: &'static str,
    pub export: RuntimeExportSurface,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
