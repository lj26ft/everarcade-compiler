# EverArcade Developer Portal v0.1

This scaffold is the first operational builder-facing portal model. It is
non-authoritative: records are derived from protocol-facing actions and roots,
not from direct mutation of runtime, settlement, or civilization state.

## Layout

- `dashboard/` — portal overview and root index.
- `projects/` — project registry records and lifecycle actions.
- `games/` — project-to-game creation flow, runtime manifest, and deployment manifest records.
- `deployments/` — local, lease, and federation deployment status records.
- `assets/` — asset, inventory, vault asset, and marketplace asset registry records.
- `wallets/` — authority, XRPL wallet, Xaman, and settlement account observations.
- `civilizations/` — civilizations, worlds, regions, governance, and economies.
- `marketplace/` — listings, sales, royalties, revenue, and settlement events.
- `gpu/` — GPU marketplace provider, capacity, assignment, artifact, and settlement intent views.
- `analytics/` — derived platform activity metrics.
- `onboarding/` — shortest path from new developer to first deployment.
- `records/` — deterministic roots for audit and replay.
