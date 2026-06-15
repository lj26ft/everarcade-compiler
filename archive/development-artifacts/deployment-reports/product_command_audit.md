# EverArcade Product Command Audit

## Summary

EverArcade now exposes a product-oriented facade under `everarcade` while preserving the existing runtime, creator, deployment, and diagnostic surface under `everarcade advanced ...` for operators and maintainers that still need low-level commands.

## Command Inventory

| Surface | Examples | Classification | Recommendation |
| --- | --- | --- | --- |
| Product facade | `doctor`, `new`, `add-rustrig`, `run`, `package`, `rehearse`, `deploy`, `validate`, `release-gate`, `status`, `stage-contract`, `artifacts-check` | Product | Keep top-level and document as the canonical path. |
| Legacy game commands | `install-game`, `list-games`, `inspect-game`, `run-game`, `start-game`, `new-game`, `package-game` | Advanced | Hide behind `everarcade advanced`; map common workflows to product commands. |
| Creator commands | `editor`, `replay-ui`, `inspect-entity`, `import-assets`, `hot-reload`, `package-content`, `creator-dashboard` | Scaffold | Keep available through `advanced` until GUI/creator flows stabilize. |
| Runtime status commands | `runtime-*status`, `runtime-*health`, `runtime-*recovery`, `runtime-package-*` | Advanced | Keep as diagnostics; summarize through `everarcade status` and `doctor`. |
| Validation scripts | `scripts/run_*_validation.sh`, `scripts/release_validate.sh`, `scripts/run_security_validation.sh` | Internal | Wrap canonical subsets with `everarcade validate` and validation scripts. |
| Deployment scripts | `scripts/generate_evernode_packages.sh`, `scripts/run_hotpocket_contract_rehearsal.sh`, `scripts/run_evernode_*` | Internal | Wrap package, rehearsal, staging, and future live deployment behind product commands. |
| Operator scripts | `scripts/everarcade_start.sh`, `scripts/install_runtime.sh`, `scripts/export-diagnostics.sh`, cluster and recovery scripts | Advanced | Keep for operations runbooks; aggregate health in `everarcade status`. |
| Generated artifact utilities | `scripts/check_no_generated_artifacts_tracked.sh`, cleaning/hash scripts | Internal | Expose policy through `everarcade artifacts-check`. |

## Hide/Deprecate Recommendations

- Prefer `everarcade new` over `new-game` and `init-game`.
- Prefer `everarcade run` over `start`, `start-game`, and `run-game` for the default developer loop.
- Prefer `everarcade package` over direct package generation scripts.
- Prefer `everarcade rehearse` over direct HotPocket rehearsal scripts.
- Prefer `everarcade validate --profile ...` over ad hoc validation script selection.
- Keep renderer, history, federation, and low-level runtime domains as scaffold/advanced commands until production hardening is complete.
