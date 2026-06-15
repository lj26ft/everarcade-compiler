# Product Command Readiness

| Capability | Status | Notes |
| --- | --- | --- |
| Product facade | Ready | Top-level commands route through `src-bin-everarcade/src/product.rs`. |
| Advanced namespace | Ready | `everarcade advanced <legacy-command>` forwards to the existing command dispatcher. |
| Doctor | Ready | Checks cargo, vendor/offline metadata, manifests, rustrigs, artifact policy, and state layout. |
| New game | Ready | Creates Arena Vanguard-compatible starter manifests. |
| Rustrig workflow | Partially Ready | Updates local game, package, and studio metadata; richer package graph integration remains future work. |
| Run | Scaffold | Initializes deterministic local state/replay markers without changing runtime internals. |
| Package | Ready | Wraps deterministic EverNode package generation. |
| Rehearse | Ready | Wraps HotPocket contract rehearsal. |
| Deploy | Scaffold | Supports dry-run and stage-contract; live EverNode deployment remains intentionally unavailable. |
| Validate profiles | Partially Ready | Provides deterministic product profiles; deeper test execution is delegated to validation scripts. |
| Release gate | Partially Ready | Produces release reports and archive; security/runtime audits remain script-backed gates. |
| Status | Scaffold | Summarizes health using deterministic frontend-ready output. |
| JSON output | Ready | Doctor, status, validate, package, rehearse, deploy, stage-contract, release-gate, and artifact checks support `--json`. |
