# Workspace Integration Closure

- CLI/runtime linkage stabilized with canonical dependency `src-bin-everarcade -> execution-core`.
- Runtime public API usage in CLI is sourced via `execution_core::runtime::export_governance` only.
- Workspace closure governance surfaces are exposed via runtime export governance audit structures.
