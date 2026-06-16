# What Is A Continuity Engine?

A continuity engine lets worlds accumulate history.

Instead of treating every patch, server move, or release as a reset, EverArcade is designed so a world can carry its story forward.

## Why continuity matters

Continuity turns a game space into a living place. Players can trust that their actions have context, consequences, and memory.

## Examples

- Buildings age and become landmarks.
- Economies evolve as players produce, trade, and consume resources.
- Institutions persist through leadership changes.
- Cultures emerge from repeated player decisions and shared history.

## Player-facing value

Continuity means a world can become worth returning to because it remembers what players did together.

## How Continuity Is Represented

The player-facing idea is simple: the world remembers. The technical representation is a chain of deterministic evidence that lets a later runtime prove where the world came from.

### State roots

A state root is a compact commitment to world state at a specific execution height. It does not replace the state archive, but it gives verifiers a stable value to compare after replay.

### Receipt roots

A receipt root summarizes execution receipts for a window. Receipts are the audit trail for accepted inputs, deterministic execution results, and emitted effects.

### Lineage

Lineage links package roots, checkpoint roots, receipt roots, state roots, and runtime version boundaries. A world with lineage can explain not only what its current state is, but how it arrived there.

### Checkpoints

Checkpoints are restoration anchors. They reduce the amount of history that must be replayed during recovery, but they remain meaningful only when tied back to earlier lineage evidence.

### Replay windows

A replay window is the range of inputs and receipts that a verifier re-executes from a known checkpoint or genesis state. Short windows make operator recovery fast; longer windows improve audit confidence and archive reconstruction.

### World history

World history is the ordered record of packages, inputs, receipts, checkpoints, state roots, and continuity roots. It is not just lore or a database dump; it is the evidence chain that lets independent infrastructure agree on the world.

### Restoration

Restoration loads a verified checkpoint, replays the required window, compares roots, and resumes only when evidence matches. A restore that cannot reproduce expected roots should be treated as a fork or incident.

### Long-lived evolution

Long-lived worlds need upgrades, migrations, and hosting changes. Continuity allows those transitions to be explicit: the old package and state root lead into a documented upgrade boundary, which then produces new receipts, checkpoints, and roots under the next runtime or world package.
