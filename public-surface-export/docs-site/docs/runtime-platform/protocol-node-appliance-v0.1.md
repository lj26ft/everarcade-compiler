# Protocol Node Appliance v0.1

## Purpose

The Protocol Node Appliance turns the installable EverArcade Runtime Appliance into an operable single-node lifecycle appliance. It proves that an operator can initialize a node, start it, run a civilization workload marker, checkpoint it, replay it, restore it, stop it, restart it, and diagnose it without adding protocol logic.

This milestone validates node operations only. Networking, peer-to-peer transport, consensus, HotPocket, Evernode, renderer, GPU runtime, and XRPL RPC remain out of scope.

## Node Layout

The canonical node root is `node/`:

```text
node/
  config/
  state/
  worlds/
  journals/
  checkpoints/
  backups/
  reports/
  logs/
```

Repository-level lifecycle reports are written to `reports/`. The node also mirrors those reports into `node/reports/` so the appliance carries its own operational evidence.

## Initialization

Run:

```bash
bash scripts/node_init.sh
```

Initialization creates the canonical layout, writes `node/config/node.env`, creates runtime state in `node/state/runtime_state.env`, creates a default world state, starts the node journal, and writes `node/node_manifest.json`.

Expected output includes:

```text
Node Initialized
Node Initialization: PASS
```

Report: `reports/node_initialization_report.txt`.

## Start

Run:

```bash
bash scripts/node_start.sh
```

Start loads configuration, loads the latest checkpoint when present, loads world state, marks the runtime as running, and appends a start event to the node journal.

Expected output includes:

```text
Node Running
Node Start: PASS
```

Report: `reports/node_start_report.txt`.

## Stop

Run:

```bash
bash scripts/node_stop.sh
```

Stop persists runtime state, flushes journals with `sync`, writes a shutdown report, and marks the node stopped.

Expected output includes:

```text
Node Stopped
Stop: PASS
```

Report: `reports/node_stop_report.txt`.

## Status

Run:

```bash
bash scripts/node_status.sh
```

Status displays:

```text
Node Status
Runtime Status
Latest Checkpoint
Latest Replay Root
Latest Continuity Root
World Status
```

Expected healthy output includes:

```text
Node Healthy
Node Status: PASS
```

## Checkpoint

Run:

```bash
bash scripts/node_checkpoint.sh
```

Checkpoint creates a checkpoint file under `node/checkpoints/`, verifies it by hashing it, records the latest checkpoint pointer, and writes the checkpoint root to the report.

Expected output includes:

```text
Checkpoint: PASS
```

Report: `reports/node_checkpoint_report.txt`.

## Replay

Run:

```bash
bash scripts/node_replay.sh
```

Replay consumes the latest checkpoint, world activity, and civilization activity log. It computes and records a replay root in `node/state/latest_replay_root`.

Expected output includes:

```text
Replay PASS
Replay: PASS
```

Report: `reports/node_replay_report.txt`.

## Restore

Run:

```bash
bash scripts/node_restore.sh
```

Restore copies the latest checkpoint, journals, and world state into a timestamped backup/restore directory, computes a continuity root, records it in `node/state/latest_continuity_root`, and marks runtime state as restored.

Expected output includes:

```text
Restore PASS
Restore: PASS
```

Report: `reports/node_restore_report.txt`.

## Doctor

Run:

```bash
bash scripts/node_doctor.sh
```

Doctor validates directories, config, state, checkpoints, journals, reports, and Runtime Appliance availability. Runtime Appliance availability is satisfied by an installed runtime binary or by the repository runtime wrappers used to operate an appliance.

Expected healthy output includes:

```text
Node Healthy
Doctor: PASS
```

Unhealthy output includes `Node Unhealthy` plus actionable errors.

## Reports

The appliance writes these reports:

```text
reports/node_initialization_report.txt
reports/node_start_report.txt
reports/node_stop_report.txt
reports/node_checkpoint_report.txt
reports/node_replay_report.txt
reports/node_restore_report.txt
reports/protocol_node_appliance_certification_report.txt
```

Reports are mirrored into `node/reports/` after each lifecycle phase that writes a report.

## PASS Criteria

Protocol Node Appliance v0.1 passes when:

1. Node initialization succeeds.
2. Node start succeeds.
3. Node status reports `Node Healthy`.
4. Checkpoint creation and verification succeed.
5. Replay reports `Replay PASS`.
6. Restore reports `Restore PASS`.
7. Node stop succeeds.
8. Doctor reports `Node Healthy`.
9. Certification reports `Protocol Node Appliance: PASS`.

## FAIL Criteria

The appliance fails when any required node directory, config, state file, manifest, journal, checkpoint, report, runtime wrapper, replay root, or continuity root is missing or invalid. Failure output should include actionable remediation, usually to rerun initialization or the missing lifecycle step.

## Relationship To Runtime Appliance

The Runtime Appliance answers whether the runtime can be installed. The Protocol Node Appliance builds on that milestone by proving that the installed or repository-wrapped runtime can be operated as a node with lifecycle state, journals, checkpoints, replay roots, restore roots, diagnostics, and certification evidence.

## Relationship To Future HotPocket Integration

HotPocket integration is a future layer. This appliance intentionally keeps networking and consensus disabled in `node/config/node.env` so future HotPocket work can attach to a stable node lifecycle without changing protocol behavior in this milestone.
