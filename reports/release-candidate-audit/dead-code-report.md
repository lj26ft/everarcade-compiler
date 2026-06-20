# Dead Code Report

## Inventory decisions

| Area | Classification | Decision | Notes |
|---|---:|---:|---|
| `crates/canonicalizer-kernel` | ACTIVE | KEEP | Minimal deterministic canonicalization kernel. |
| `crates/transport-core` | ACTIVE | KEEP | Transport core used by runtime-facing work. |
| `crates/rustrigs/*` | ACTIVE | KEEP | RustRig invariant library surfaced by `world rustrigs`. |
| `runtime/everarcade-runtime` | ACTIVE | KEEP | Local runtime used by `world run` and verification flows. |
| `runtime/hotpocket-adapter` | SUPPORTED | KEEP | Adapter boundary for documented HotPocket integration. |
| `runtime/games/arena-vanguard` | SUPPORTED | KEEP | Public demo runtime game. |
| `examples/reference-certified-world-v1` | SUPPORTED | KEEP | Reference example for public review. |
| `examples/world-creation-flow/frontier-validation` | SUPPORTED | KEEP | Frontier validation world. |
| `examples/template-worlds` and Creator SDK templates | SUPPORTED | KEEP | Public template library. |
| `examples/reference-certified-world` | OBSOLETE | ARCHIVE | Superseded by `reference-certified-world-v1`. |
| `examples/reference-world-package` | OBSOLETE | ARCHIVE | Superseded by v1 reference package and Creator SDK world package flow. |
| `examples/reference-world` / `examples/reference-world-registry` | UNKNOWN | ARCHIVE | Useful historical fixtures but not primary onboarding flow. |
| `runtime/*-proof` HotPocket/XRPL/continuity proof harnesses | EXPERIMENTAL | KEEP | Keep as proof scaffolds; do not document as first-run path. |
| `runtime/renderer-client`, `runtime/replay/history`, `runtime/*federation*` | EXPERIMENTAL | KEEP | Scaffold-level domains per release instructions. |
| Root legacy helper scripts (`runtime-*`, `scripts/*` not in README path) | UNKNOWN | ARCHIVE | Keep out of primary docs until each script has a current owner and flow. |
| Committed `node_modules/` under runtime proof harnesses | OBSOLETE | REMOVE | Vendor/generated dependency artifacts should not be committed for public release. |
| Committed `dist/` proof outputs | OBSOLETE | REMOVE | Generated release/demo outputs should be regenerated or archived separately. |

## CLI surface

Primary public commands: `everarcade world templates`, `world rustrigs`, `world init`, `world run`, `world package`, `world verify`, and `world deploy`.

Legacy commands remain present for compatibility: `new`, `build`, `test`, `package`, `certify-world`, `verify-world-certificate`, `launch-local`, `execute-local`, `execute-template`, `execute-guest`, `play-*`, `deploy`, and `publish`. Decision: KEEP as compatibility surface, but ARCHIVE from public first-run docs.

## Conclusion

No runtime code was removed in this milestone. Public release cleanup should prioritize removing committed dependency/output artifacts and archiving superseded examples after maintainers confirm no tests depend on them.
