# Simple Village Reference World Report

Simple Village is a noncombat reference world assembled through the same Runtime Assembly Engine path as Catacombs: canonical request, profile graph, contribution graph, deterministic merge, symbol/reference resolution, `PtwRuntimeIrV1`, and canonical `world.evr` lowering.

## Profiles

- `everarcade.world.ptw-full@1.0.0`
- `everarcade.genre.social-exploration-baseline@1.0.0`
- `everarcade.topology.simple-village@1.0.0`
- `everarcade.biome.temperate-village@1.0.0`
- `everarcade.encounter.none@1.0.0`
- `everarcade.proof.live-replay-ceremony@1.0.0`
- `everarcade.projection.village-web@1.0.0`
- `everarcade.economy.founding-world-sandbox@1.0.0`

## Topology, entities, and actions

The topology declares five regions: village square, market lane, workshop yard, meeting hall, and village gate. The initial world declares eight archetypes and six entities: merchant, workbench, notice board, storage chest, village gate, and herb resource node.

Enabled primitives are identity, movement, inventory, items, and interactions. Explicitly disabled primitives are health, combat, spawning, encounters, and progression. Enabled actions are `player.join`, `entity.move`, `interaction.activate`, `item.pickup`, and `item.use`; combat and respawn actions are absent.

## Invariants, limits, and proof

Generic invariants cover unique entity IDs, valid archetype references, topology-bounded positions, valid ownership, inventory capacity, nonnegative item quantities, declared-action-only mutation, resource bounds, and valid interaction transitions. Limits include player, entity, topology, action payload, actions per tick, movement delta, inventory, item stack, world variable, invariant, interaction target, and resource-node caps.

Proof policy uses checkpoint, journal, root, replay, receipt coverage, and live replay ceremony declarations.

## Deferred

Advanced crafting, real merchant settlement, NPC AI, persistent social memory, housing, land ownership, governance, resource economy, quests, polished Village projection, and live deployment are deferred.

## Hashes and validation results

- Profile graph hash: `sha256:44783f1747c41219bad2c13dd707f6e01a0087e1a9835b59f429f853efb7cc38`
- Contribution graph hash: `sha256:3ecda27ddde383bf3b7e33d0ce4564978f76dd8a6623b2f4b69ada100b2be38c`
- Merged contribution hash: `sha256:03a222b9eb99b44f22fcf1f30b99e27dfb94e0b89aa339bce43d870051230a5e`
- Reference graph hash: `sha256:bfe42551d9b1d2215ec487bf3ac19ccc3b912fbd26404a5921792d19cf47dc81`
- Runtime IR hash: `sha256:973a50219e1cfdd65b2f36386066d5912fd4dfe9712bea3e4b7748a4459f7ab1`
- Runtime bundle hash: `2abcb66d72f2d685a9bf0975ff39aab130105375037579ff4ee52c911dbac518`
- World contract hash: `142c3ee92e3ecec9a601b2a12cbe955dee102772050b9036271f482ec9c6fe32`
- Package hash: `419324180fece5dc06ccae4da464c297c953336321e122b80597a6dbe17446f3`

Validation passed for deterministic double generation, disabled primitive serialization, absent combat/respawn actions, projection-only Runtime IR hash stability, and cross-world hash differences.
