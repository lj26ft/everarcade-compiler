# Repo Reorganization Plan for v0.1.x

## Goal
Reduce onboarding and maintenance friction while preserving deterministic/offline/replay-safe architecture.

## Migration table

| current path | proposed path | reason | risk | migration order | tests required after move |
|---|---|---|---|---|---|
| `execution-core/` | `crates/execution-core/` | clearer crate topology | medium (workspace path edits) | 1 | `cargo test -p execution-core` |
| `everarcade-host/` | `crates/everarcade-host/` | consistent crate grouping | medium | 2 | `cargo test -p everarcade-host` + runtime build |
| `src-bin-everarcade/` | `crates/everarcade-cli/` | remove name/path mismatch | medium | 3 | `cargo test -p everarcade-cli` |
| `runtime/config/*` | `runtime/config/*` (keep) | already aligned | low | n/a | bootstrap validate |
| `scripts/*.sh` | `scripts/{dev,release,validation,diagnostics}/` | discoverability | medium | 4 | smoke all moved scripts |
| `docs/*.md` | `docs/{architecture,runtime,onboarding,release}/` | canonical docs map | low | 5 | link checker/manual read |
| `test_vectors/` | `test_vectors/` (keep) | aligns target layout | low | n/a | vector-loading tests |
| `compiler/kernel/*.py` | `tools/compiler-kernel/*` or archive | role clarity | medium | 6 | python smoke if retained |
| `compiler/kernel/__pycache__/*.pyc` | removed | generated artifact cleanup | low | 0 | git clean check |

## Suggested migration order
0. Cleanup generated artifacts + add ignore rules.
1. Create root README and canonical docs index.
2. Restructure scripts into subfolders while keeping compatibility shims.
3. Move crates into `crates/` with Cargo workspace updates.
4. Consolidate duplicate docs.
5. Re-run full validation matrix and release build.

## Safety constraints
- Do not change deterministic runtime behavior during path-only moves.
- Use temporary shim scripts to avoid breaking operators.
- After each phase, run at least:
  - `cargo fmt --all --check`
  - `cargo test --workspace`
  - `bash scripts/build_runtime_release.sh`
  - `bash scripts/validate_clean_vm_bootstrap.sh`
