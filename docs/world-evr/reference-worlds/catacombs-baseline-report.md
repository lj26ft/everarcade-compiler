# Catacombs Baseline Migration Report

## Scope

This phase proves that **EverArcade Catacombs: The Endless Gate** assembles through the generic Runtime Assembly Engine from pinned request/profile/module/contribution data. It does **not** claim that the Authority repository migration is complete. The Authority inventory file was not available in this isolated workspace, so the mapping document records the required follow-up cross-check.

## Canonical request

* Fixture: `fixtures/worlds/endless-gate-catacombs/world-create-request.json`
* World ID: `everarcade-catacombs-endless-gate`
* World name: `EverArcade Catacombs: The Endless Gate`
* Seed material: `everarcade-catacombs-endless-gate:v1`

## Profile identities

* `everarcade.world.ptw-full@1.0.0`
* `everarcade.genre.arpg-baseline@1.0.0`
* `everarcade.topology.catacombs-baseline@1.0.0`
* `everarcade.biome.catacombs@1.0.0`
* `everarcade.encounter.catacombs-baseline@1.0.0`
* `everarcade.proof.live-replay-ceremony@1.0.0`
* `everarcade.projection.arpg-web@1.0.0`
* `everarcade.economy.founding-world-sandbox@1.0.0`

## Runtime IR summary

* Entity archetypes: 5 (`player`, `skeleton-sentinel`, `keeper`, `ancient-gate`, `relic-pedestal`)
* Item archetypes: 1 (`gate-relic`)
* Initial entities: 4 (`keeper-entity`, `ancient-gate-entity`, `gate-relic-entity`, `sentinel-001`)
* Regions: 3 (`entry-hall`, `gate-chamber`, `relic-vault`)
* Spawn points: 5 (`player-entry`, `keeper-post`, `gate-threshold`, `relic-dais`, `enemy-watch`)
* Enabled primitives: `combat`, `encounters`, `health`, `identity`, `interactions`, `inventory`, `items`, `movement`, `progression`, `spawning`
* Enabled actions: `combat.attack`, `entity.move`, `interaction.activate`, `item.pickup`, `item.use`, `player.join`, `world.respawn`

## Hashes from repeated generation

Two package generations under `target/reference-worlds/endless-gate-catacombs-a` and `target/reference-worlds/endless-gate-catacombs-b` were byte-identical.

| Hash | Value |
|---|---|
| Profile graph hash | `sha256:8ad60ec72f49888657f4838169b5eda037f193a8bc3b4cd6800b762c15e346a9` |
| Contribution graph hash | `sha256:01571d294db951d00234d96b69bb90cee13fd1afd67d9aa91188e85b272a772a` |
| Merged contribution hash | `sha256:10043220f9d24e56d156955b62dcd2d908bcee4c5cf0a92351a9f2c1f72ad714` |
| Reference graph hash | `sha256:c62dc045ab6c82a8e799d542d22fd1606092ffc6a873bfc822ccb52ae56848ae` |
| Runtime IR hash | `sha256:a600cbbc5780c8e044167247ae3456eea22e54470721929d19fc0949ae47d693` |
| Runtime bundle hash | `9d5f0603ec5f0d22115d2ae9b7f428942a63e26033bfe2161eb311e1e836d9dc` |
| World contract hash | `9caf734a2ebdf0e485e098862f286de921164937585689d3367a903282f55ea5` |
| Package hash | `27a9403a7449cfdfb75d64556aae6b14f78066bcb556325be2ed456cf32176c3` |

## Validation results

* Catacombs assembles through `world-evr create` from the canonical request fixture.
* Repeated package generation is byte-identical.
* Reference validation tests reject a removed region declaration.
* Hash-sensitivity tests prove authoritative entity mutation changes `runtime_ir_hash`.
* Projection separation tests prove projection-only label changes do not alter `runtime_ir_hash` while changing package contribution evidence.

## Deferred features

The following remain intentionally deferred: endless dungeon cycles, dynamic monster spawning, advanced AI, boss phases, loot scaling, difficulty progression, full combat loop, polished ARPG projection, public Shawn/Dane ceremony, Evernode deployment, and public cross-machine proof completion.
