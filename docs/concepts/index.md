# Concepts

Core EverArcade concepts:

- **World** — a packaged deterministic game/simulation state machine.
- **Runtime** — the local execution host that records receipts, journals, replay material, and state.
- **Replay verification** — re-executing recorded inputs to validate deterministic outcomes.
- **Continuity** — preserving world state across checkpoints, archive, restore, and continued execution.
- **Ownership and identity** — protocol-facing concepts for assets, wallets, players, and operators; most live settlement surfaces are not production-ready.

Useful references:

- [System architecture](../architecture/02-system-overview.md)
- [World runtime](../architecture/world/world-runtime.md)
- [Runtime capabilities](../runtime-capabilities.md)
- [Settlement boundary](../runtime/settlement_boundary.md)
