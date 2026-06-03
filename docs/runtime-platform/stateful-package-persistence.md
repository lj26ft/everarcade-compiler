# Stateful Package Persistence Certification v0.1

## Purpose

Stateful Package Persistence Certification validates that the EverArcade Runtime
Platform can execute a deterministic package, persist its resulting state,
restart, restore that state, continue execution, and replay the full lifecycle to
the same continuity root.

This certification proves long-lived deterministic state continuity. It does not
add networking, federation, XRPL integration, game deployment, world sharding, or
new runtime features.

## Validation Workflow

Run the certification from the repository root:

```bash
bash scripts/certify_stateful_package_persistence.sh
```

The certification follows this chain:

```text
Runtime Bootstrap
      ↓
Initial Deterministic Package Execution
      ↓
Capture State Root A and Execution Root A
      ↓
Write Persistence Checkpoint
      ↓
Simulate Runtime Restart
      ↓
Load Checkpoint and Restore State
      ↓
Continue Deterministic Execution
      ↓
Capture State Root B and Execution Root B
      ↓
Validate Continuity Root
      ↓
Replay Full Lifecycle
      ↓
Compare Replay Continuity Root
```

The script writes a concise report at:

```text
reports/stateful_package_persistence_report.txt
```

## Persistence Guarantees

A passing result demonstrates that:

- runtime bootstrap certification is available and passing,
- initial deterministic package execution produces a stable state root,
- a checkpoint record is created for the persisted state,
- the persistence record names the same checkpoint identifier, and
- checkpoint identity is derived from package metadata and persisted roots.

## Restoration Guarantees

A passing result demonstrates that:

- the simulated restart does not reuse live in-memory state,
- the checkpoint can be loaded after restart,
- the restored state root equals the persisted state root, and
- the runtime recovery path can continue from the restored checkpoint.

## Continuity Guarantees

A passing result demonstrates that:

- continued execution starts from the restored state root,
- post-restore execution advances to State Root B,
- no state divergence is detected between persisted and restored state, and
- the continuity root binds the checkpoint, restored state, and continued
  execution output.

## PASS Criteria

The certification passes only when all result categories pass:

- Bootstrap: PASS
- Persistence: PASS
- Restoration: PASS
- Continuity: PASS
- Replay: PASS
- Overall Result: PASS

Replay requires:

```text
Replay Continuity Root == Continuity Root
```

## FAIL Criteria

The certification fails if any required category fails, including:

- runtime bootstrap certification is unavailable or not passing,
- package metadata required for deterministic execution is unavailable,
- checkpoint creation or persistence-record writing fails,
- restored state differs from persisted state,
- continued execution does not advance from the restored state, or
- replay produces a different continuity root.

The report still records roots, checkpoint identity, category statuses, and the
overall result for diagnosis.

## Relationship To Deterministic Package Execution

Deterministic Package Execution proves that a package produces repeatable roots
for a fixed input sequence. Stateful Package Persistence extends that proof by
splitting execution across a checkpoint and restart boundary, then confirming
that replay of the complete lifecycle reproduces the same continuity root.
