# 08. SDK Development

## Purpose

The SDK helps creators build deterministic packages and helps tools interact with runtime artifacts. It improves ergonomics without moving authority out of the runtime.

## SDK Areas

| Area | Crates / Paths | Role |
|---|---|---|
| Core SDK | `sdk/everarcade-sdk` | game/session/replay/runtime helpers |
| World SDK | `sdk/everarcade-world-sdk` | world authoring boundaries |
| Entity SDK | `sdk/everarcade-entity-sdk` | entity modeling helpers |
| Simulation SDK | `sdk/everarcade-simulation-sdk` | deterministic simulation helpers |
| Economy SDK | `sdk/everarcade-economy-sdk` | economy abstractions |
| Governance SDK | `sdk/everarcade-governance-sdk` | governance abstractions |
| Client bridge | `sdk/client-bridge` | bridge to clients and projections |

## Development Rules

- SDK APIs should make deterministic choices easy and nondeterministic choices hard.
- SDK dev runtimes are not production authority.
- SDK examples should point to runtime verification and packaging gates.
- SDK docs can explain usage, but architecture ownership stays in this canonical set.
