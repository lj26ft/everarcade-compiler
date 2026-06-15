# Final Refactor Recommendation

## Summary

Do not perform a major file move before live deployment. The repository already contains runtime, control-plane, rustrig, Studio, deployment, docs, examples, templates, and scripts domains. The bounded longevity work is to clarify ownership, add facade commands, and make authority boundaries explicit.

## Suggested future layout

```text
crates/runtime/
crates/sdk/
crates/control-plane/
crates/studio/
crates/cli/
rustrigs/
deployment/
scripts/
docs/
examples/
templates/
```

## Bounded refactors to do after launch gate

| Refactor | Scope | Benefit | Risk |
| --- | --- | --- | --- |
| CLI facade aliases | Add `run`, `package`, `deploy`, `add-rustrig` wrappers around existing commands. | Simplifies developer flow without moving internals. | Low. |
| Authority path constants | Centralize `state/world`, `state/replay`, `state/checkpoints`, `state/receipts`, `state/rustrigs`, `state/packages`, `state/anchors`, `state/operator`. | Prevents path drift across scripts/runtime/docs. | Low. |
| Control-plane provider adapter trait | Define an EverNode provider boundary with a modeled provider and a live provider. | Allows real EverNode integration without rewriting orchestration. | Medium. |
| Observability export adapter | Add JSON/Prometheus/OpenTelemetry export outside consensus. | Makes production dashboards possible. | Low/medium. |
| Settlement service boundary crate or protocol | Define intent/receipt API shared with external settlement service. | Keeps XRPL/Xahau custody out of runtime. | Medium. |
| Studio guided workflow shell | Route GUI actions through the same manifest/package/runtime commands as CLI. | Reduces creator complexity. | Medium. |
| Package/rustrig manifest normalization | Ensure one canonical source of package and rustrig hashes. | Improves reproducibility and deploy safety. | Medium. |

## Refactors to avoid before live deployment

- Moving all crates into `crates/` in one patch.
- Rewriting rustrig ABI or replay serialization.
- Adding live XRPL vault custody inside the runtime.
- Promoting renderer/history/federation scaffolds to authority.
- Replacing existing deployment tests with a new full-workspace validation strategy.

## Recommended sequencing

1. Freeze the authority model and deployment docs.
2. Add CLI aliases and hide advanced help.
3. Add provider adapter interfaces while keeping the modeled provider as the default.
4. Wire real telemetry export and alert routing.
5. Integrate live EverNode API/process provider.
6. Integrate external settlement service via intent/receipt records.
7. Only then consider physical repo reorganization into `crates/`.
