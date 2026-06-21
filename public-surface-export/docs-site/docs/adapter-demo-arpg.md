# Adapter Demo ARPG v1

This demo is an original browser-first, Diablo-like ARPG pattern. It uses simple generated placeholder shapes and does not include proprietary game names, art, rules, or assets.

## Message

**Keep your engine. Keep your assets. Replace the authoritative server.**

The browser client renders the dungeon and captures input, while the EverArcade-style adapter owns authoritative world mutations: `combat.attack`, `inventory.pickup`, `inventory.transfer`, `world.spawn`, `position.move`, and `continuity.record_event`.

## Layout

- `examples/adapter-demo-arpg/` — playable browser demo and Creator SDK project manifest.
- `adapters/browser-arpg/` — reusable adapter surface for the authoritative world loop.
- `reports/adapter-demo-arpg/report.txt` — acceptance report.

## Adapter surface

```js
connectWorld()
submitInput()
readProjection()
verifyWorld()
disconnect()
```

Inputs: `move`, `attack`, `pickup`, `open_chest`, `return_later`, `restore`, and `migrate`.

Outputs include projection frames, player state, enemy state, loot state, inventory, receipts, roots, and world memory.

## Demo loop

1. Open `examples/adapter-demo-arpg/index.html` in a browser or serve the repo with a static file server.
2. Move with WASD/arrow keys.
3. Attack with Space or click.
4. Pick up loot with E.
5. Press R to simulate restore and M to simulate migration.
6. Watch receipts, state roots, receipt roots, and world memory update in the HUD.

## Packaging and verification

```bash
node creator-sdk/cli/everarcade.mjs world package --project examples/adapter-demo-arpg
node creator-sdk/cli/everarcade.mjs world verify --project examples/adapter-demo-arpg
node creator-sdk/cli/everarcade.mjs world deploy --project examples/adapter-demo-arpg
```

Expected verification output:

```text
WORLD VERIFY: PASS
```

The package flow writes `dist/world.evr`, `dist/certification/world-package-certificate.json`, `dist/certification/independent-proof-recheck.json`, and `dist/deployment.json`.
