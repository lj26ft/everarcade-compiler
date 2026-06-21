# Adapter Demo ARPG: 3-minute script

1. “This is a normal top-down ARPG client.” Show the browser canvas, movement, enemies, health bars, and placeholder dungeon room.
2. “The client renders and handles input.” Move with WASD/arrow keys and explain that the canvas is only a renderer/input shell.
3. “EverArcade owns the authoritative world state.” Point at receipts, roots, and world memory in the HUD.
4. Kill an enemy with Space/click. Explain that `combat.attack` resolves deterministic damage outside the client authority.
5. Pick up loot with E. Explain that ownership and inventory mutation are adapter-authoritative.
6. Show world memory, receipt count, state root, and receipt root changing after each mutation.
7. Run `node creator-sdk/cli/everarcade.mjs world package --project examples/adapter-demo-arpg` and show `dist/world.evr`.
8. Run `node creator-sdk/cli/everarcade.mjs world verify --project examples/adapter-demo-arpg` and show `WORLD VERIFY: PASS`.
9. Press R and M in the demo, or describe restore/migration artifacts, to show dungeon history is preserved.
10. Close: “Existing games can become verifiable worlds.”
