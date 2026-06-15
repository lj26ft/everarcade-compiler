# EverArcade Repository Map

This map explains major subsystems, ownership boundaries, dependency relationships, and intended audiences.

## Canonical entry points

- `README.md` — short public introduction and start path.
- `docs/` — canonical documentation source.
- `MATURITY.md` — subsystem maturity and current reality check.
- `archive/` — historical milestone, validation, certification, handoff, and development artifacts.

## Core runtime and execution

- `execution-core/` — core deterministic execution engine used by runtime proof paths. Audience: runtime contributors.
- `runtime/` — runtime packages, HotPocket experiments, XRPL anchor proofs, public API manifests, and runtime adapters. Audience: runtime contributors and operators.
- `everarcade-host/`, `everarcade-abi/`, `contract-api/` — host, ABI, and contract boundary code. Audience: runtime and contract developers.

Dependency direction: world packages and contracts depend on ABI/contract definitions; runtime hosts execute packages and emit evidence; verification consumes runtime evidence.

## Creator and world authoring

- `creator-sdk/` — CLI and templates for creating, building, testing, packaging, and playing local worlds. Audience: developers.
- `templates/`, `examples/`, `creator-examples/` — sample worlds and authoring examples. Audience: developers and contributors.
- `rustrigs/` — gameplay domain modules such as combat, inventory, movement, economy, crafting, dialogue, quests, and world state. Audience: world developers.

## Operations and deployment

- `scripts/` — validation, certification, packaging, install, and operator utility scripts. Audience: contributors and operators.
- `deploy/`, `deployment/`, `evernode/`, `public-testnet/` — deployment layouts, Evernode boundaries, and public-network scaffolds. Audience: operators; see `MATURITY.md` before use.
- `archive/` — moved historical reports and prior validation records. Audience: maintainers and auditors.

## Federation, history, and networking

- `federation/` — topology, synchronization, recovery, settlement, identity, and replay scaffold. Audience: runtime researchers and future federation contributors.
- `gateway/`, `frontend-gateway/`, `crates/transport-core/` — transport and gateway code. Audience: networking contributors.

These areas are scaffold-level unless current validation evidence says otherwise.

## Frontend, renderer, and portals

- `renderer/` — projection and rendering scaffold. Audience: renderer contributors.
- `frontend/`, `developer-portal/`, `player_sessions/`, `xaman/` — portal, wallet, and session surfaces. Audience: application contributors; not canonical proof path.
- `clients/` — reference clients.

## Settlement, marketplaces, and GPU

- `hooks/` — Xahau/XRPL hook boundary experiments.
- `gpu/` — GPU worker, queue, verification, and marketplace scaffolds.
- `creator-marketplace/`, `commercial-revenue/`, `game-discovery/`, `registry/` — marketplace, discovery, registry, and revenue concepts.

These directories are primarily reference or scaffold material today.

## Documentation ownership

Use `docs/DOCUMENTATION_POLICY.md` for where new docs belong. Subsystem README files should point to canonical docs instead of becoming independent portals.
