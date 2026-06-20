# Validation Output

Commands were validated from the source checkout with `node creator-sdk/cli/everarcade.mjs world ...` because the package binary is not globally linked in this container.

```text
$ node creator-sdk/cli/everarcade.mjs world templates
Arena        Fast combat worlds
Frontier     Persistent survival worlds
Settlement   Economy and governance worlds
Social       Community worlds
Civilization Long-term continuity worlds

$ node creator-sdk/cli/everarcade.mjs world rustrigs
combat           CERTIFIED  Deterministic attacks, damage, health, and combat resolution.
inventory        CERTIFIED  Item ownership, slots, transfers, and equipment state.
market           CERTIFIED  Creator-safe marketplace listing and exchange flows.
governance       CERTIFIED  World policy, proposals, roles, and rule changes.
identity         CANDIDATE  Player and entity identity surfaces.
movement         CANDIDATE  Position, bounds, and deterministic movement updates.
resources        CANDIDATE  Resource spawning, harvesting, and balances.
crafting         CANDIDATE  Recipes, inputs, outputs, and production timers.
structures       CANDIDATE  Buildings, placement, ownership, and durability.
quests           CANDIDATE  Objectives, progression, and rewards.
continuity       CANDIDATE  Save continuity, upgrades, and long-lived world lineage.
operations       CANDIDATE  Local operations, health, diagnostics, and deployment readiness.

$ node creator-sdk/cli/everarcade.mjs world init --template frontier --name frontier-world --dir /tmp/ea-v11
World: PASS (frontier-world)
Next: read docs/creator-sdk/world-project-map.md, then run everarcade world run

$ CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs world run --project /tmp/ea-v11
Local Runtime Launch: PASS (frontier-world-world)

$ node creator-sdk/cli/everarcade.mjs world package --project /tmp/ea-v11
World Package: PASS (frontier-world)
World Package File: dist/world.evr

$ node creator-sdk/cli/everarcade.mjs world verify --project /tmp/ea-v11
Test: PASS (frontier-world)
World Package: PASS (frontier-world)
World Package File: dist/world.evr
World Package Certification: PASS
Independent Proof Re-check: PASS
WORLD VERIFY: PASS

$ node creator-sdk/cli/everarcade.mjs world deploy --project /tmp/ea-v11
Deploy: PASS (frontier-world)
```
