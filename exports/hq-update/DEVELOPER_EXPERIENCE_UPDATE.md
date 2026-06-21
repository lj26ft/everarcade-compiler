# Developer Experience Update

## Validated Workflow

```bash
everarcade world templates
everarcade world rustrigs
everarcade world init
everarcade world run
everarcade world package
everarcade world verify
everarcade world deploy
```

## Real Execution Output

Command sequence executed from repository root on 2026-06-21 UTC with a temporary `frontier-world` project:

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

$ node creator-sdk/cli/everarcade.mjs world init --template frontier --name frontier-world --dir "$PROJECT"
World: PASS (frontier-world)
Next: read docs/creator-sdk/world-project-map.md, then run everarcade world run

$ CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs world run --project "$PROJECT"
Build: PASS (frontier-world)
World Package: PASS (frontier-world)
World Package File: dist/world.evr

$ node creator-sdk/cli/everarcade.mjs world package --project "$PROJECT"
Build: PASS (frontier-world)
World Package: PASS (frontier-world)
World Package File: dist/world.evr

$ node creator-sdk/cli/everarcade.mjs world verify --project "$PROJECT"
Test: PASS (frontier-world)
World Package: PASS (frontier-world)
World Package File: dist/world.evr
World Package Certification: PASS
Independent Proof Re-check: PASS
WORLD VERIFY: PASS

$ node creator-sdk/cli/everarcade.mjs world deploy --project "$PROJECT"
Deploy: PASS (frontier-world)
```

## Result

- WORLD VERIFY: PASS
- Deploy: PASS
