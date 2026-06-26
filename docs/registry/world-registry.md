> **Repository boundary:** Registry material in this repository is limited to neutral world identity, metadata, and local validation examples.
>
> Hosted discovery, ranking, reputation, curation, and commercial registry operation are outside the scope of this open-source reference implementation.
>
# World Registry & Discovery Network

The World Registry is the canonical, machine-readable discovery index for EverArcade worlds. It combines world identity, portable metadata, auditable trust signals, governance visibility, contributor discovery, reputation, activity, lineage, capabilities, and health into a replayable registry record.

## World identity

A world is addressed by a globally unique `world_id` such as `frontier.evr`, `arena-vanguard.evr`, `kingdoms.evr`, or `civilization.evr`. The identifier is stable across operator changes, restores, migrations, and governance changes because mutable operational fields are carried as metadata while the registry record keeps the same `world_id`, `world_hash`, and `registry_root` identity anchors.

Every world record includes:

```json
{
  "world_id": "frontier.evr",
  "world_hash": "sha256:frontier-genesis-root",
  "world_name": "Frontier",
  "description": "A sandbox RPG frontier world with player settlements, economy, and open contributor projects.",
  "created_at": "2026-01-01T00:00:00.000Z",
  "operator_id": "operator:frontier-foundation",
  "governance_model": "constitutional-council",
  "registry_root": "registry-root:frontier-v1"
}
```

## Metadata standard

Registry metadata is portable and replayable JSON. Common fields include description, category, tags, operator, governance model, population, contributors, treasury status, and proof status. Standard categories are `MMO`, `RPG`, `Simulation`, `Strategy`, `Education`, `Commerce`, `Governance`, `Social`, and `Sandbox`.

## Discovery index and APIs

The Creator SDK exposes registry queries for search, lookup, contributor discovery, and lineage exploration:

```bash
everarcade world search rpg
everarcade world search governance
everarcade world lookup frontier.evr
everarcade world contributors frontier.evr
everarcade world lineage frontier.evr
```

Search supports browse/filter/sort behavior by matching names, descriptions, categories, and tags, then sorting by vitality signals. A project can publish a local `world.registry.json` with a top-level `worlds` array to override the built-in fixture during local development.

## Trust, governance, and abuse resistance

World records display auditable trust signals: replay verification, restore verification, migration verification, operator history, governance activity, and audit roots. Registry governance mitigates fake worlds, sybil worlds, spam registrations, and misleading metadata through verification proofs, reputation signals, governance records, and challenge mechanisms.

Governance visibility includes constitution links, maintainers, reviewers, council members, and governance activity so players and contributors can understand how a world is governed before joining.

## Contributor and reputation visibility

Worlds publish contributor manifests with wanted roles, open projects, reward models, and contribution opportunities. Contributor records expose merged contributions, maintainer status, review history, and reputation scores so worlds can recruit trustworthy builders.

## Population, lineage, capabilities, and health

Registry records expose active players, active contributors, merge activity, governance activity, economic activity, origin world, forks, merges, migration history, restore events, installed capability modules, treasury health, governance health, contributor retention, player retention, and verification status.

A canonical landing page can be generated from the same record with world name, description, governance model, population, contributors, trust signals, treasury status, and actions for Play, Contribute, Verify, and Fork.
