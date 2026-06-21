# EverArcade Public Demonstration Script

## Demonstration thesis

```text
Small game feel. Big world-continuity idea.
```

Arena Vanguard is presented as a playable proof-of-world demo: a frontier scout clears a creature camp, the camp becomes a remembered ruin, and replay/restore/migration preserve that remembered world.

## Audience takeaway

Within the first minute, the viewer should understand:

- I can move.
- I can fight creatures.
- I can loot.
- The world records what happened.
- Returning later shows the world aged.
- Replay, restore, and migration preserve the same remembered world.
- Verification roots exist underneath the game view.

## Tone

Lead with game feel, then explain continuity. Avoid claims that the page is a production game, commercial MMO, live HotPocket authority, wallet flow, XRPL/Xahau settlement, or marketplace. Call it a local playable developer-preview.

## Scene 1: Enter the frontier

**Screen action:** Open `arena-vanguard-playable.html` from a local server. Move the scout with WASD or arrow keys.

**Narration:**

> This is a tiny PvE proof-of-world. You are a frontier scout. The goal is simple: clear the creature camp and return later to see what the world remembers.

**On-screen emphasis:** Top HUD objective, large canvas, region labels, minimap.

## Scene 2: Fight the camp

**Screen action:** Move into the Frontier camp. Attack with Space or click. Let the viewer see enemy chase, hit flash, health bars, attack arcs, and damage numbers.

**Narration:**

> Player actions are local journal inputs. Movement and combat advance tick, receipts, and roots.

## Scene 3: Kill the camp leader

**Screen action:** Defeat the Camp Leader.

**Visual result:** Camp clear banner appears, the camp becomes a remembered ruin marker, and World Memory records First Kill and Camp Cleared.

**Narration:**

> The important moment is not the kill itself. The world records that this place changed.

## Scene 4: Loot

**Screen action:** Press E near the sparkle loot.

**Visual result:** Loot count increases, First Loot appears in World Memory, and a checkpoint refreshes.

**Narration:**

> Loot is also remembered as a deterministic state transition.

## Scene 5: Return later

**Screen action:** Return to the Citadel and press L.

**Visual result:** World Age advances, resources regenerate, the ruin appears older, and the timeline says: `You returned to an older world.`

**Narration:**

> This is the thesis: returning later does not reset the camp. The world aged, and the cleared camp is still part of history.

## Scene 6: Replay

**Screen action:** Press R.

**Visual result:** The local journal visibly replays and the verification strip shows `REPLAY VERIFIED`.

**Narration:**

> The local journal can replay from genesis and arrive at the same remembered world.

## Scene 7: Restore

**Screen action:** Press C.

**Visual result:** The checkpoint restores and execution continues with `RESTORE VERIFIED`.

**Narration:**

> Restore is not a reset. It is a continuation from a checkpoint while preserving world memory.

## Scene 8: Migration

**Screen action:** Press M.

**Visual result:** The World Memory panel shows Migration Preserved and the verification strip shows `MIGRATION VERIFIED`.

**Narration:**

> The local source world package moves to a destination world without losing the remembered ruin.

## Scene 9: Close

**Screen action:** Leave the canvas on the aged ruin, memory panel, timeline, and verification strip.

**Narration:**

> Arena Vanguard is not claiming live settlement here. It is demonstrating the continuity primitive: a playable world whose history can be remembered, replayed, restored, migrated, and verified.

End card:

```text
ARENA VANGUARD PLAYABLE REMAKE: PASS
```
