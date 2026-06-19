# Arena Vanguard: Reference Projection Visual Spec

Arena Vanguard is presented as a **reference projection** and **certified world viewer**, not a game demo or production client. The interface is a dark, infrastructure-grade operator console with neon proof accents.

Required viewer comprehension in under 60 seconds:

- The world state is changing.
- State roots and world hashes are always visible.
- Operators independently agree on roots.
- Replay, restore, and migration preserve continuity.
- The projection is read-only; the world is the artifact.

Core surfaces:

- Arena map with grid, boundary, region labels, coordinate ticks, spawn zones, operator observation ring, players, enemies, neutral entities, trails, health rings, and last-action glow.
- Certification overlay with tick, epoch, shortened state root, shortened world hash, receipt count, Tier 1 PASS, Tier 2 PASS, Projection PASS, and World Package PASS.
- Operator cards for Operator A, Operator B, and Operator C with status, state root, world hash, last tick, and verification result.
- Replay scrubber at Tick 0, 25, 50, 75, and 100 with join, move, attack, trade, vote, restore, and migration markers.
- Event feed for player, combat, inventory, market, governance, receipt, root, restore, and migration events.
