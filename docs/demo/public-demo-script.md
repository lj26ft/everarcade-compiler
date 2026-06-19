# EverArcade Public Demonstration Script

## Demonstration Thesis

```text
The World Is The Artifact.
```

This public demonstration is designed to make one idea obvious in under sixty seconds and undeniable within five minutes: an EverArcade world is not a session, server, or save file. It is a deployable, verifiable, replayable, restorable, and migratable artifact that continues across operators.

## Audience Takeaway

By the end of the demo, a viewer with no prior EverArcade context should understand that:

- Worlds are deployable.
- Worlds are verifiable.
- Worlds are portable.
- Worlds survive operator changes.
- Worlds survive migration without reset or rollback.

## Core Contrast

Traditional games:

```text
Player
↓
Server
↓
Game
```

EverArcade:

```text
World
↓
Package
↓
Verification
↓
Deployment
↓
Continuation
```

## Tone

The narration should feel infrastructure-grade: calm, precise, and confident. Avoid hype language, fantasy framing, casino-like language, and gameplay-first claims. The operator should sound like they are demonstrating a reliable deployment system for persistent worlds.

## Certification Overlay

Keep this overlay visible throughout the demonstration:

```text
Tier 1: PASS
Tier 2: PASS
Projection Runtime: PASS
World Package: PASS
```

## Scene 1: Build World

**Screen action:** Show the command terminal and package panel.

```bash
everarcade new-world arena-vanguard
```

**Visual result:** A `world.evr` artifact appears with package structure:

```text
arena-vanguard/
├── world.evr
├── manifest.json
├── runtime.wasm
├── state/
├── receipts/
└── checkpoints/
```

**Narration:**

> In EverArcade, the world itself is a deployable artifact.

**On-screen emphasis:** Highlight `world.evr`, then pulse the World Hash and State Root fields in the left panel.

## Scene 2: Certify World

**Screen action:** Run certification.

```bash
everarcade certify-world
```

**Visual result:** Display deterministic certification checks.

```text
Package Integrity: PASS
Determinism Profile: PASS
Replay Plan: PASS
Certificate: generated
```

**Narration:**

> Before deployment, the world can be independently verified.

**On-screen emphasis:** The certification overlay remains small but visible. The certificate icon attaches to `world.evr`.

## Scene 3: Deploy World

**Screen action:** The right operator panel activates Operator A, Operator B, and Operator C.

```text
Operator A: ONLINE
Operator B: ONLINE
Operator C: ONLINE
```

**Narration:**

> Three operators. One world.

**On-screen emphasis:** The same World Hash is copied to all three operators. Status indicators turn bright green only after all three load the same package.

## Scene 4: Live World

**Screen action:** The projection runtime starts. The center panel shows minimalist state-focused activity:

- movement paths,
- combat intent and resolution,
- inventory deltas,
- market trades,
- governance votes.

**Live overlay:**

```text
Tick: 1842
Epoch: 12
State Root: 0x9f4c...a21e
Receipt Count: 493
```

**Narration:**

> The runtime is a projection of the world state. Gameplay events become receipts. Receipts advance the world.

**On-screen emphasis:** Receipts flow from the center panel into the left World Overview panel and then synchronize to the right Operator View.

## Scene 5: Verification

**Screen action:** Highlight operator roots.

```text
Operator A Root: 0x9f4c...a21e
Operator B Root: 0x9f4c...a21e
Operator C Root: 0x9f4c...a21e
```

**Animated confirmation:**

```text
ROOTS MATCH
```

**Narration:**

> Independent operators reached the same world state.

**On-screen emphasis:** Root synchronization lines converge into one bright green verification bar.

## Scene 6: Replay

**Screen action:** Stop live execution, dim the runtime, and run replay.

```bash
everarcade replay
```

**Visual result:** History replays from receipts and ends at the same State Root.

```text
Replay Start Root: 0x1a02...41bf
Replay End Root:   0x9f4c...a21e
Live End Root:     0x9f4c...a21e
REPLAY VERIFIED
```

**Narration:**

> The world can be replayed from its history and arrive at the same state.

## Scene 7: Restore

**Screen action:** Restore a checkpoint and resume simulation.

```text
Checkpoint: epoch-12.tick-1800
Restore Root: 0x7bd0...337c
Continuation Root: 0x9f4c...a21e
RESTORE VERIFIED
```

**Narration:**

> Restore does not mean reset. The world continues from a verified checkpoint.

**On-screen emphasis:** The tick counter resumes from the restored checkpoint instead of returning to zero.

## Scene 8: Migration

**Screen action:** Show migration from Operator A to Operator D.

```text
Operator A
    ↓
Migration Package
    ↓
Operator D
```

**Visual result:** Operator D joins with the same World Hash and State Root.

```text
Operator D: ONLINE
Migration Root: 0x9f4c...a21e
MIGRATION VERIFIED
```

**Narration:**

> The world is not trapped inside an operator. It can move and continue.

## Scene 9: Final Reveal

**Screen action:** Clear the runtime and show the final capability ladder.

```text
CREATE
CERTIFY
VERIFY
DEPLOY
REPLAY
RESTORE
MIGRATE
CONTINUE
```

Then reveal:

```text
BUILD WORLDS.
NOT JUST GAMES.
```

End card:

```text
PUBLIC DEMONSTRATION CERTIFICATION: PASS
```
