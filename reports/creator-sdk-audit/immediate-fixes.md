# Immediate Fixes (< 1 day)

These are small friction reductions that avoid new architecture.

## CLI aliases

1. Add `everarcade world init` as an alias for `new`.
2. Add `everarcade world run` as the recommended alias for `play-local`.
3. Add `everarcade world package` as an alias for `package`.
4. Add `everarcade world verify` as an alias that runs the best current local checks or prints the exact current sequence.
5. Add `everarcade world deploy` as an alias for `deploy`.
6. Add `everarcade world templates` or `everarcade world init --list-templates`.

## Documentation

1. Put the shortest “first world” flow at the top of Creator SDK docs.
2. Add a table: “If you want X, choose template Y.”
3. Add a table mapping creator concepts to files: metadata, genesis, contract, continuity, projection.
4. Add “Package means…” glossary near the quick start.
5. Add “Deployment maturity” note beside deploy instructions.
6. Document that `package` auto-builds if needed.

## Naming shortcuts

1. Add `frontier` as a documented alias for the recommended beginner template if that is the target product language.
2. Generate or document a `world.evr` artifact name even if it wraps the existing `dist/runtime-package` layout.
3. Rename or label generated files so `dist/package.json` is not confused with Node `package.json`.

## RustRig discovery

1. Add `everarcade world rustrigs` to list combat, inventory, governance, movement, resources, crafting, structures, factions, quests, continuity, operations, market, and world.
2. Add certified/alpha/scaffold labels in the list.
3. Add docs examples for combat, inventory, and governance additions.

## Time To First World impact

These changes should reduce **Time To First World** by removing vocabulary detours and making the happy path copy/pasteable.
