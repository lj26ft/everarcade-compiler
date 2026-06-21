# 02. Platform Vision

## Vision

EverArcade is a verifiable runtime network for persistent game worlds. The long-term platform lets creators publish deterministic worlds, operators host them, players interact through clients, and auditors verify world history from receipts, checkpoints, and replay artifacts.

## Why This Platform Exists

The platform exists because games and simulations with persistent value need stronger guarantees than conventional game servers provide. A world should be recoverable after host failure, explainable after a dispute, portable across operators, and auditable without trusting a renderer or database snapshot.

## North Star

A creator should be able to build a world package, certify it, deploy it to a runtime node, observe deterministic execution, recover from failure, upgrade safely, and eventually federate authority across hosts.

## Platform Pillars

- **Deterministic runtime:** authoritative execution produces reproducible roots.
- **Evidence-first operation:** receipts, journals, checkpoints, manifests, and replay archives are operational artifacts.
- **Composable SDK:** creator-facing SDKs expose world, entity, economy, simulation, governance, and client bridge boundaries.
- **Federated hosting:** peer coordination is built around verification and recovery rather than blind trust.
- **Projection layer:** renderers and portals stream state projections without owning authority.
- **Release certification:** releases must pass deterministic, recovery, upgrade, and deployment gates before production use.

## Product Boundaries

EverArcade is not a renderer, a wallet, a marketplace, or a generic cloud hosting product by itself. Those systems can be built around the runtime, but the core platform is the deterministic execution and operational evidence layer.

## Roadmap Direction

The platform should advance in this order:

1. close v0.1 runtime gates;
2. harden beta operator workflows;
3. certify production single-host operation;
4. harden multi-host federation;
5. complete commercial Evernode hosting;
6. integrate XRPL settlement boundaries;
7. add ZK proof boundaries;
8. launch creator marketplace flows.
