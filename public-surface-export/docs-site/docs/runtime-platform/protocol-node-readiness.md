# Protocol Node Readiness v0.1

## Purpose

Protocol Node Readiness validates that the EverArcade Runtime Platform can act as
the recoverable operational foundation for a protocol node. It does not add
networking, federation, XRPL integration, Evernode deployment, game hosting, or
new runtime features.

This milestone focuses on node operation after deployment readiness has already
been proven by runtime bootstrap certification and release artifact validation.

## Operational Guarantees

A passing certification demonstrates that the current runtime can:

- bootstrap from the certified runtime bundle,
- persist runtime state through the runtime storage surfaces,
- recover through the existing recovery validation surfaces,
- verify replay equivalence through existing replay report validation, and
- restore or validate checkpoints through existing checkpoint validation.

Together these checks prove the operational chain:

```text
Runtime Bootstrap
      ↓
Runtime Lifecycle
      ↓
Persistence
      ↓
Recovery
      ↓
Replay Equivalence
      ↓
Node Ready
```

## Validation Workflow

Run the node readiness certification script from the repository root:

```bash
bash scripts/certify_protocol_node_readiness.sh
```

The script runs these existing validation surfaces:

1. Runtime bootstrap certification:
   `bash scripts/certify_runtime_bootstrap.sh`
2. Runtime persistence and lifecycle tests:
   `cargo test -p everarcade-runtime --tests --offline --locked`
3. Runtime recovery validation:
   `bash scripts/run_runtime_recovery_validation.sh` and targeted runtime
   recovery tests.
4. Replay report validation:
   `bash scripts/run_execution_replay_validation.sh --offline --locked`
5. Checkpoint restore validation:
   `bash scripts/run_checkpoint_restore_validation.sh --offline --locked`

The script writes a concise report to:

```text
reports/protocol_node_readiness_report.txt
```

## PASS Criteria

Protocol Node Readiness passes only when all result categories pass:

- Bootstrap: PASS
- Persistence: PASS
- Recovery: PASS
- Replay: PASS
- Checkpoint: PASS
- Overall Result: PASS

## FAIL Criteria

Protocol Node Readiness fails when any required category fails, including:

- runtime bootstrap certification failure,
- persistence or lifecycle test failure,
- recovery validation failure,
- replay report validation failure, or
- checkpoint restore validation failure.

The report still records every category status so operators can identify the
failed node-readiness surface.

## Relationship To Runtime Bootstrap

Runtime Bootstrap Certification proves that a certified runtime bundle can be
verified, restored, and bootstrapped. Protocol Node Readiness builds on that
proof by validating the operational node loop after bootstrap: persistence,
recovery, replay equivalence, and checkpoint continuity.
