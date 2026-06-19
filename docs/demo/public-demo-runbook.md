# Arena Vanguard Public Demo Runbook

## Primary public playable

Use the playable proof-of-world page as the first public impression:

```bash
python3 -m http.server 5174 --directory release/demo-production
# open http://127.0.0.1:5174/arena-vanguard-playable.html
```

The page is intentionally a small 2D PvE game, not a dashboard. It demonstrates movement, combat, loot, persistent world memory, world aging, replay, restore, migration, and verification roots in one local developer-preview loop.

## Safety boundary

The playable demo is local and read-only with respect to any live authority. It must not submit HotPocket inputs, sign transactions, touch wallets, use XRPL/Xahau settlement, or claim commercial marketplace operation. The verification strip labels the hashing path as `DEMO ROOT FALLBACK — canonicalizer integration pending` until a production browser canonicalizer is wired.

## Operator flow

1. Serve `release/demo-production` locally.
2. Open `arena-vanguard-playable.html`.
3. Confirm the top HUD shows World Age, Tick, Objective, Player Health, and Loot Count.
4. Move the scout with WASD or arrow keys from the Citadel to the Frontier camp.
5. Attack with Space or click until the camp leader dies.
6. Pick up loot with E.
7. Return to the Citadel and press L to advance world age.
8. Confirm the camp remains visible as a remembered ruin and the World Memory panel still shows First Kill, Camp Cleared, First Loot, and Returned Later.
9. Press R and confirm `REPLAY VERIFIED`.
10. Press C and confirm `RESTORE VERIFIED`.
11. Press M and confirm `MIGRATION VERIFIED` plus Migration Preserved in World Memory.

## Controls

- WASD / arrow keys: move
- Space / click: attack
- E: pick up loot
- Q: show or hide the objective card
- L: return later / advance world age
- R: replay local journal
- C: restore checkpoint
- M: migrate local world package
- T: optional deterministic trade receipt
- V: optional deterministic vote receipt

## Validation commands

```bash
node --check release/demo-production/arena-vanguard-playable.js
python3 -m json.tool release/demo-production/demo-world-seed.json >/dev/null
python3 -m http.server 5174 --directory release/demo-production
```

During the manual browser test, complete: move, attack, kill camp leader, loot, return later, replay, restore, migrate. The browser console should remain free of errors.

## Secondary technical pages

The older projection/dashboard pages may still be used for technical inspection, but they are no longer the first public playable experience. Treat renderer, history, and federation surfaces as scaffold-level runtime domains unless the runtime authority is explicitly wired for the environment under test.
