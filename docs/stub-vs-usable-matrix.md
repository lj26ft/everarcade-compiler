# Stub vs Usable Matrix (2026-05-23)

| component | status | usable today? | test coverage | blockers | next action |
|---|---|---|---|---|---|
| execution-core | prototype | yes (core lib compiles/tests) | high (extensive suite) | domain IO leaks in some modules | isolate pure core from IO adapters |
| WASM engine | prototype | yes | medium-high (`wasm_*` tests) | warning-level API/export ambiguity | tighten module exports |
| host ↔ WASM memory bridge | prototype | yes | medium | boundary docs incomplete | document ABI invariants |
| deterministic receipt layer | production-useful | yes | high | cross-module naming sprawl | centralize receipt docs |
| state diff layer | prototype | partial | medium | ownership unclear across modules | map single canonical state-diff path |
| runtime operations layer | prototype | partial | medium | many scripts, duplicate semantics | collapse into smaller command surface |
| packaging layer | production-useful | yes | medium (script smoke) | deterministic timestamp fallback not strict | require SOURCE_DATE_EPOCH in CI |
| Evernode manifest layer | prototype | partial | low-medium | docs and ownership split | add one owner module + docs |
| CLI | prototype | yes | medium | crate path/name mismatch, fmt drift | move to `crates/everarcade-cli`, enforce fmt |
| host runtime | prototype | yes | medium | very large `main.rs` orchestration surface | split command handlers |
| replay diagnostics | prototype | partial | medium | duplicated docs/tools | consolidate replay docs |
| federation recovery | scaffold | partial | low-medium | many modules, uncertain runtime integration depth | add integration scenario tests |
| topology scaling | scaffold | partial | low-medium | limited explicit operator playbook | add deterministic topology runbook |
| docs | stale | partial | n/a | no root README, duplicate guides | publish canonical docs index |
| validation scripts | production-useful | yes | medium | ordering assumptions (vendor first) | add preflight bootstrap checks |
