# Runtime Namespace Governance

## Canonical Runtime APIs

- `execution_core::runtime::validation::runtime::ValidationDagRuntime`
- `execution_core::runtime::ci::runtime::CiExecutionHistoryRuntime`
- `runtime::renderer_client::history::timeline::HistoricalReplayTimeline`

## Ownership and Lineage

Public runtime symbols are owned by the module that defines their concrete runtime struct/type.
Alias-style module re-exports are disallowed; explicit symbol exports or glob exports from leaf modules are used instead.

## Module Boundaries

- Renderer/history/federation domains are scaffold-level, reconstruction-only runtime domains.
- Validation and CI domains expose canonical runtime integration APIs.
- Replay transport and history domains preserve non-authoritative renderer semantics.
