# EverArcade Player Gateway v0.1

The Player Gateway is the canonical player-facing entry point for discovering games, creating identities, joining worlds, managing inventories, viewing marketplaces, observing civilizations, and launching sessions without exposing protocol internals.

The gateway is intentionally non-authoritative. It observes, launches, displays, and records player preferences, but it does not mutate runtime state, bypass authority, bypass settlement, or bypass replay.

## Layout

- `profiles/` — portable player identity registry and authority mappings.
- `characters/` — game-scoped character identities and progression views.
- `games/` — installed, published, recent, and favorite games.
- `sessions/` — login, game, world, and civilization session records.
- `inventory/` — player-facing item, equipment, container, vault, and marketplace asset views.
- `marketplace/` — listings, purchases, sales, creator assets, and player assets with no live settlement.
- `civilizations/` — observation-only worlds, regions, settlements, governance, and events.
- `social/` — friends, guilds, groups, and contacts without production messaging.
- `authority/` — XRPL/Xaman authority status, wallet mappings, settlement history, and authorization events without key custody.
- `analytics/` — replayable player participation metrics.
- `replay/` — replay equivalence surface for player activity, characters, inventory, marketplace, and sessions.
- `launcher/` — browse, install, launch, update, and remove records for future game discovery.
- `metrics/` — player success metrics for registrations, characters, installs, play, marketplace activity, and retention.
- `records/` — gateway layout and aggregate roots.
