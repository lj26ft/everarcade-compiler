# Persistent World Certification v0.1

## Purpose

Persistent World Certification proves that the EverArcade Runtime Platform can host a deterministic long-lived world across genesis, evolution, checkpointing, restart, restoration, continued evolution, epoch transition, and replay without continuity divergence.

This is a certification surface only. It does not add runtime features, networking, federation, XRPL integration, economies, or vault ownership.

## World Guarantees

A PASS result means:

- A deterministic World Identifier is derived from the runtime surface and world manifest.
- World Genesis produces a deterministic Genesis Root.
- Epoch 0, Epoch 1, and Epoch 2 roots evolve from prior roots and do not collapse to the same value.
- Continued Epoch 3 and Epoch 4 roots advance from the restored checkpoint root.
- The final World Continuity Root binds the full lifecycle transcript.

## Epoch Guarantees

A PASS result means:

- Epoch lineage is preserved from Genesis through Epoch 4.
- The restored root equals the checkpointed Epoch 2 root.
- Epoch 3 and Epoch 4 advance after restoration.
- No epoch rollback is accepted.
- No continuity divergence is accepted.

## Persistence Guarantees

A PASS result means the checkpoint record contains:

- World Identifier.
- Checkpoint Identifier.
- Checkpoint Epoch.
- Persisted State Root.
- Genesis and Epoch 0 through Epoch 2 roots.

The checkpoint is valid only when these fields match the deterministic lifecycle values.

## Replay Guarantees

A PASS result means replaying the complete lifecycle regenerates the same root sequence and produces a Replay Continuity Root equal to the original World Continuity Root.

The replay covers:

1. Genesis.
2. Evolution.
3. Checkpoint.
4. Restart and restore.
5. Continued evolution.
6. Epoch transition.

## PASS Criteria

The certification passes only when all of the following are true:

1. Runtime bootstrap certification passes.
2. Deterministic world genesis succeeds.
3. Epoch 0, Epoch 1, and Epoch 2 roots evolve.
4. Checkpoint creation and integrity validation succeed.
5. Runtime restoration loads the checkpoint and recovers state.
6. Epoch 3 and Epoch 4 continue from the restored state.
7. Epoch lineage is preserved without rollback or divergence.
8. Replay Continuity Root equals World Continuity Root.

## FAIL Criteria

The certification fails if any of the following occur:

- Runtime bootstrap certification fails.
- World manifest or runtime package descriptor is unavailable.
- Genesis cannot produce a deterministic root.
- Epoch roots fail to evolve.
- Checkpoint material is missing or mismatched.
- Restoration does not recover the checkpointed state.
- Continued epochs roll back or diverge.
- Replay cannot reproduce the World Continuity Root.

## Relationship To Tenant Runtime Certification

Tenant Runtime Certification proves isolated tenant ownership and replay continuity for groups of packages.

Persistent World Certification uses the same deterministic persistence and replay model but raises the scope from tenant isolation to sovereign world continuity. It validates that one long-lived world can survive restart, restoration, and epoch evolution while preserving a single continuity root. This makes it the prerequisite for economic ledger, vault ownership, federation synchronization, and civilization runtime certifications.
