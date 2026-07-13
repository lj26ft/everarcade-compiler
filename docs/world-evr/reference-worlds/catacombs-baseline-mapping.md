# Catacombs Baseline Mapping

Authority inventory note: `../everarcade-authority/docs/PTW_RUNTIME_EXTRACTION_INVENTORY.md` was not present in this isolated workspace, so this compiler-side mapping records the certified baseline concepts named by the Phase F task and marks the external inventory cross-check as pending.

| Source Authority concept | Classification | New profile/module | Contribution namespace | Runtime IR destination | Status |
|---|---|---|---|---|---|
| PTW runtime contract | primitive configuration | `everarcade.world.ptw-full@1.0.0` | `runtime` | `runtime_contract`, numeric/coordinate/tick/id policies | Authoritative migrated |
| Catacombs map bounds and blocked cells | topology declaration | `everarcade.topology.catacombs-baseline@1.0.0` | `topology` | `topology_graph` | Authoritative migrated |
| Entry Hall, Gate Chamber, Relic Vault | topology declaration | `everarcade.topology.catacombs-baseline@1.0.0` | `regions` | `regions` | Authoritative migrated |
| Player entry, keeper post, gate threshold, relic dais, enemy watch | topology declaration | `everarcade.topology.catacombs-baseline@1.0.0` | `spawn_points` | `spawn_points` | Authoritative migrated |
| Generic player spawn template | initial entity policy | `everarcade.world.ptw-full@1.0.0` | `runtime` | `player_state_template` | Authoritative migrated |
| Player archetype | entity archetype | `everarcade.encounter.catacombs-baseline@1.0.0` | `entity_archetypes` | `entity_archetypes` | Authoritative migrated |
| Keeper archetype and initial entity | entity archetype / initial entity | `everarcade.encounter.catacombs-baseline@1.0.0` | `entity_archetypes`, `entities`, `world_variables` | `entity_archetypes`, `initial_entities`, `world_variables` | Authoritative migrated; projection dialogue deferred |
| Ancient Gate archetype/entity | interaction declaration | `everarcade.encounter.catacombs-baseline@1.0.0` | `entity_archetypes`, `entities`, `transitions` | `entity_archetypes`, `initial_entities`, `transition_bindings` | Authoritative migrated using `interaction.activate` |
| Gate relic item | item archetype / initial entity | `everarcade.encounter.catacombs-baseline@1.0.0` | `item_archetypes`, `entities` | `item_archetypes`, `initial_entities` | Authoritative migrated |
| Skeleton sentinel | entity archetype / initial entity | `everarcade.encounter.catacombs-baseline@1.0.0` | `entity_archetypes`, `entities`, `encounters` | `entity_archetypes`, `initial_entities`, `encounters` | Authoritative baseline migrated; advanced AI deferred |
| identity/movement/health/combat/inventory/items/interactions/spawning/encounters/progression | primitive configuration | `everarcade.genre.arpg-baseline@1.0.0` | `primitives` | `primitive_configurations` | Authoritative explicit enablement |
| player.join, entity.move, interaction.activate, item.pickup, item.use, combat.attack, world.respawn | action declaration | `everarcade.genre.arpg-baseline@1.0.0` | `actions` | `actions` | Authoritative generic actions migrated |
| Generic bounded transitions | transition binding | `everarcade.genre.arpg-baseline@1.0.0`, `everarcade.encounter.catacombs-baseline@1.0.0` | `transitions` | `transition_bindings` | Authoritative migrated |
| Required invariant set | invariant | `everarcade.genre.arpg-baseline@1.0.0` | `invariants` | `invariants` | Authoritative migrated |
| Baseline limits | invariant / primitive configuration | `everarcade.world.ptw-full@1.0.0` | `limits` | `limits` | Authoritative migrated |
| Checkpoint, journal, root, replay policies | proof policy | `everarcade.world.ptw-full@1.0.0`, `everarcade.proof.live-replay-ceremony@1.0.0` | `proof` | `checkpoint_policy`, `journal_policy`, `root_policy`, `proof_constraints` | Authoritative migrated; public cross-machine proof not claimed |
| ARPG web labels/camera/assets | projection-only metadata | `everarcade.biome.catacombs@1.0.0`, `everarcade.projection.arpg-web@1.0.0` | `projection` | Excluded from `runtime_ir_hash` | Non-authoritative migrated |
| endless dungeon cycles, dynamic spawns, advanced AI, boss phases, loot scaling, difficulty progression, full combat loop, polished projection, public Shawn/Dane ceremony, Evernode deployment | future unsupported behavior | Deferred | N/A | N/A | Deferred |
