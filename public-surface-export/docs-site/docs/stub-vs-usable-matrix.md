# Stub vs Usable Matrix (2026-05-30)

Use this matrix as the current maturity source of truth before making public v0.1 claims. Renderer, history, and federation remain scaffold-level runtime domains unless a specific end-to-end test proves otherwise.

| component | status | usable today? | test/verification signal | blockers | next action |
|---|---|---|---|---|---|
| `everarcade` product CLI | prototype | yes for local template flow | `new-game`, `start-game`, `replay-world`, `diagnostics` are wired | help surface mixes product, dev, runtime, and scaffold commands | publish `docs/CLI_QUICKSTART.md`; group or hide scaffold commands |
| Game creation template | prototype | yes | `templates/topdown-arena` copies into `runtime/games/<id>` | generated game is a smoke starter, not a complete playable package | add one `everarcade new --run` happy path and template validation |
| Game run/start flow | prototype | yes, smoke-level | writes `runtime/world/status.txt` and `runtime/replay/latest/frame-0001.json` | not yet a real interactive session loop | connect input queue, runtime client, persistence, and replay verification |
| Game packaging | prototype | partial | `package-game` writes manifest hash | package hash covers `game.toml`, not full canonical artifact tree | implement canonical package archive/hash/verify commands |
| Asset pipeline | prototype | partial | asset commands write deterministic marker manifests/hashes | no real import/transform policy, type support, or size limits | define asset schema and deterministic transform validator |
| `everarcade-host` init/run/verify | prototype | yes | host help exposes init, generate-fixture, run, verify examples | very large command surface and mixed maturity | document first-four command operator path; split handlers later |
| execution-core | production-useful prototype | yes | extensive tests and runtime API audit surfaces exist | broad public/internal boundary | freeze `execution_core::api` facade for v0.1 |
| WASM/contract runtime | prototype | partial | contract crates and host execution commands exist | ABI, memory, fuel, and isolation docs incomplete | finish ABI conformance and sandbox threat model |
| deterministic receipt/checkpoint layer | production-useful prototype | yes | receipt/checkpoint docs and tests exist | docs fragmented across many files | consolidate receipt/checkpoint reference |
| replay diagnostics | prototype | partial | `replay-world`, `inspect-simulation`, replay status commands | `docs/replay-verification.md` is placeholder-grade | write replay verification guide with corruption/recovery examples |
| runtime appliance/release scripts | production-useful prototype | yes | bootstrap/release scripts and appliance docs exist | vendor and path preconditions need stronger public messaging | add doctor/release gate entrypoint and signed checksums |
| terminal runtime client | local demo | yes | deterministic ticks, roots, validation output, replay count | scripted demo rather than user-controlled session | expose interactive input or connect to product CLI run path |
| renderer client | scaffold | partial | projection/status commands emit deterministic markers | non-authoritative; not a product renderer | integrate with replay windows and web-reference client |
| history/archive surfaces | scaffold | partial | historical query/archive commands exist as status/projection paths | one-line docs and no public runbook | create archive import/export tests and docs |
| federation/topology/recovery | scaffold | partial | status/recovery commands and modules exist | no public multiplayer trust/abuse model; user-facing runtime only | build two-node vertical-slice scenario before launch claims |
| XRPL/IPFS/Evernode integration | scaffold/prototype | partial, dry-run only | docs and gated scripts exist | live submission/key/retry semantics not audited | keep disabled by default; publish dry-run boundary |
| SDK crates | prototype | partial | multiple SDK crates and terse docs exist | creator workflow not cohesive | promote one canonical SDK starter tutorial |
| Studio/creator GUI | scaffold | no for v0.1 | `studio`/`studio-gui` present | product workflow not integrated | defer public GUI; focus CLI-first creator MVP |
| docs | mixed | partial | new CLI quickstart and launch audit added | many one-line placeholders and duplicates | move placeholders to roadmap or expand before public docs launch |
| validation scripts | production-useful prototype | yes | targeted scripts exist | script sprawl and overlapping names | document canonical `doctor` and `release_gate` commands |
| host placeholder tests | stub | no | multiple tests assert placeholder invariants | false confidence for federation/governance/recovery domains | replace with scenario tests or mark roadmap/ignored |
