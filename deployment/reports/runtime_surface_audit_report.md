# Runtime Surface Audit Report

## Namespace continuity validation
- historical replay fabric imports now use explicit module paths in test integration.
- wildcard `pub use *` dependence removed from historical replay tests.
- replay/runtime symbol continuity verified for timeline, archive, compression, branch, corruption, observer restore, and anchor surfaces.

## Runtime surface classifications
- execution_core::runtime::validation => Production
- execution_core::runtime::ci => ActiveIntegration
- renderer_client::history => Scaffold
- renderer_client::federation => Scaffold
- renderer_client::transport_runtime => Scaffold
- renderer_client::history::archive_hydration => ActiveIntegration
- renderer_client::history::compression => ActiveIntegration
- renderer_client::history::proof_materialization => ActiveIntegration

## Export ownership summary
- `history/mod.rs` exports now narrowed to deterministic replay integration contract symbols.
- stale broad re-exports removed from test-facing surface.

## Disconnected/stale surface scan
- no unresolved replay symbol disconnects detected in replay fabric and transport runtime tests.
