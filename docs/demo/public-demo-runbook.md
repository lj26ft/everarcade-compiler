# EverArcade Public Demonstration Runbook

## Purpose

This runbook defines the operator-facing procedure for staging the EverArcade public demonstration. The demo prioritizes world continuity, portability, replayability, verification, and migration over gameplay complexity.

## Required Outputs

The demonstration must produce these report artifacts under `reports/public-demo/`:

```text
demo-report.txt
operator-report.txt
replay-report.txt
restore-report.txt
migration-report.txt
```

## Preflight Checklist

Before recording or presenting, confirm:

- The demo build uses the dark operator-oriented visual profile.
- The world name is `arena-vanguard`.
- The left panel includes live World Name, World Hash, State Root, Tick, Epoch, and Receipt Count values.
- The center projection runtime shows movement, entities, combat, inventory, market, and governance activity.
- The right operator panel shows Operator A, Operator B, and Operator C state roots, world hashes, and status values.
- The certification overlay is visible throughout.
- Every PASS state uses the success green visual treatment.
- No screen uses generic fantasy MMO framing or generic crypto UI framing.

## Demonstration Procedure

### 1. Build World

Run or simulate the build command:

```bash
everarcade new-world arena-vanguard
```

Expected visible result:

```text
world.evr created
package structure displayed
world hash generated
state root initialized
```

Record in `demo-report.txt`:

```text
[BUILD] world.evr created: PASS
[BUILD] package structure displayed: PASS
```

### 2. Certify World

Run or simulate certification:

```bash
everarcade certify-world
```

Expected visible result:

```text
Package Integrity: PASS
Determinism Profile: PASS
Replay Plan: PASS
Certificate: generated
```

Record in `demo-report.txt`:

```text
[CERTIFY] Package Integrity: PASS
[CERTIFY] Determinism Profile: PASS
[CERTIFY] Replay Plan: PASS
[CERTIFY] Certificate generated: PASS
```

### 3. Deploy Operators

Bring three operators online:

```text
Operator A: ONLINE
Operator B: ONLINE
Operator C: ONLINE
```

Expected result:

```text
all operators report identical world hash
all operators report initial state root
all operator statuses are green
```

Record in `operator-report.txt`:

```text
[OPERATOR A] world hash match: PASS
[OPERATOR B] world hash match: PASS
[OPERATOR C] world hash match: PASS
[OPERATORS] three operator deployment: PASS
```

### 4. Run Live Projection

Start the projection runtime and display state updates:

```text
movement
combat
inventory changes
market trades
governance updates
```

Expected result:

```text
tick increases
epoch visible
receipt count increases
state root updates deterministically
```

Record in `demo-report.txt`:

```text
[LIVE] projection runtime active: PASS
[LIVE] receipts emitted: PASS
[LIVE] state root updated: PASS
```

### 5. Verify Operator Roots

Compare operator state roots:

```text
Operator A Root == Operator B Root == Operator C Root
```

Expected result:

```text
ROOTS MATCH
```

Record in `operator-report.txt`:

```text
[VERIFY] operator roots identical: PASS
[VERIFY] independent state convergence: PASS
```

### 6. Replay History

Stop live execution and replay history:

```bash
everarcade replay
```

Expected result:

```text
REPLAY VERIFIED
```

Record in `replay-report.txt`:

```text
[REPLAY] receipt history loaded: PASS
[REPLAY] final root equals live root: PASS
[REPLAY] replay verified: PASS
```

### 7. Restore Checkpoint

Restore from a checkpoint and continue simulation.

Expected result:

```text
RESTORE VERIFIED
```

Record in `restore-report.txt`:

```text
[RESTORE] checkpoint loaded: PASS
[RESTORE] continuation root generated: PASS
[RESTORE] no reset detected: PASS
[RESTORE] restore verified: PASS
```

### 8. Migrate World

Migrate from Operator A to Operator D:

```text
Operator A
    ↓
Migration Package
    ↓
Operator D
```

Expected result:

```text
Operator D online
world hash unchanged
state root unchanged at migration boundary
execution continues after migration
MIGRATION VERIFIED
```

Record in `migration-report.txt`:

```text
[MIGRATION] migration package created: PASS
[MIGRATION] Operator D online: PASS
[MIGRATION] world hash preserved: PASS
[MIGRATION] state root preserved: PASS
[MIGRATION] continuation after migration: PASS
[MIGRATION] migration verified: PASS
```

## Final Acceptance Gate

The demo passes only if all of the following are true:

- The viewer can identify `world.evr` as the deployable artifact.
- Certification occurs before deployment.
- Three independent operators show matching roots.
- Replay ends at the live root.
- Restore continues without reset.
- Migration preserves world identity and continuation.
- The final screen displays the required pass statement.

Final output:

```text
PUBLIC DEMONSTRATION CERTIFICATION: PASS
```
