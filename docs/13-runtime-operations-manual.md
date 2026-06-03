# 13. Runtime Operations Manual

Audience: runtime operators. This manual describes production-intent procedures. If a command is marked partial in `11-production-readiness.md`, operators must treat it as requiring additional release-owner approval before production use.

## Command Form

The runtime binary accepts:

```bash
cargo run -p everarcade-runtime -- <command> <runtime-root> <world-id> <package-path>
```

Examples below use:

```bash
export EVERARCADE_ROOT=runtime/operator-node
export EVERARCADE_WORLD=world-001
export EVERARCADE_PACKAGE=runtime/games/2d-arena
```

## Install Runtime

1. Obtain release artifacts and manifest from the approved release bundle.
2. Verify checksums against `release/SHA256SUMS` or the release manifest.
3. Install the runtime binary, package, configuration, and service files on the target host.
4. Create runtime storage directories with operator-only write access.
5. Record runtime version, package hash, host identity, and deployment manifest.

## Configure Runtime

Required configuration values:

- runtime root;
- world identifier;
- package path;
- runtime version;
- checkpoint directory;
- journal directory;
- backup directory;
- peer configuration when federation is enabled;
- observability endpoints when available.

Configuration changes that affect authority require a checkpoint and release-owner approval.

## Start Runtime

```bash
cargo run -p everarcade-runtime -- start "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

After start, run status and verify.

## Stop Runtime

```bash
cargo run -p everarcade-runtime -- stop "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

Confirm no input admission is active before maintenance.

## Restart Runtime

```bash
cargo run -p everarcade-runtime -- restart "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

Run replay verification after restart.

## Create Backup

```bash
cargo run -p everarcade-runtime -- backup "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

Store the backup manifest, checkpoint hash, runtime version, package hash, and host identity outside the runtime root.

## Restore Backup

Restore is partial and requires release-owner approval before production use.

Procedure:

1. stop runtime;
2. copy runtime root to quarantine storage;
3. verify backup manifest and checkpoint hash;
4. restore checkpoint material into the runtime root;
5. replay journal entries after the checkpoint;
6. run `verify`, `replay-verify`, and `doctor`;
7. resume only after roots match expected values.

## Verify Replay

```bash
cargo run -p everarcade-runtime -- replay-verify "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

For reporting:

```bash
cargo run -p everarcade-runtime -- replay-report "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

For root comparison:

```bash
cargo run -p everarcade-runtime -- replay-root "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

## Recover Runtime

```bash
cargo run -p everarcade-runtime -- recover "$EVERARCADE_ROOT" "$EVERARCADE_WORLD" "$EVERARCADE_PACKAGE"
```

Recovery must select verified checkpoints or replay material. If recovery reports missing or corrupted artifacts, quarantine the runtime root and escalate.

## Upgrade Runtime

1. announce maintenance;
2. stop input admission;
3. create checkpoint;
4. create backup;
5. verify old runtime replay;
6. verify new artifact checksums;
7. install new runtime;
8. start runtime;
9. run `verify`, `replay-verify`, `replay-report`, and `doctor`;
10. retain rollback artifacts until the release owner accepts post-upgrade validation.

## Roll Back Runtime

1. stop runtime;
2. preserve current failed root for incident analysis;
3. restore pre-upgrade runtime binary and package;
4. restore pre-upgrade checkpoint if required;
5. replay to the last accepted pre-upgrade root;
6. run verification commands;
7. document incident and approval.

## Recover Federation Node

Federation recovery is partial. For any production-like federation incident:

1. remove the node from input authority if possible;
2. preserve peer manifests, receipt ranges, checkpoints, and logs;
3. compare local roots against trusted peers;
4. import only verified missing receipts or checkpoints;
5. run replay comparison;
6. rejoin only after peer policy accepts the node.

## Evernode Deployment

1. verify Evernode release certification status;
2. install provider-specific templates and manifests;
3. verify runtime artifact hashes;
4. configure storage, networking, identity, and backups;
5. start runtime;
6. run runtime validation commands;
7. archive deployment evidence.

Commercial Evernode hosting remains partial until the release certification document marks all provider gates complete.

## Troubleshooting

| Symptom | Immediate Action | Escalation |
|---|---|---|
| `status` fails | Verify runtime root and status file permissions. | Restore from checkpoint if status is corrupt. |
| `verify` fails | Stop runtime and preserve journal. | Run recovery; compare checkpoint hash. |
| `replay-verify` fails | Treat as divergence. | Quarantine artifacts and run root comparison. |
| Backup fails | Confirm checkpoint exists. | Create checkpoint and retry; inspect storage. |
| Recovery fails | Do not resume authority. | Escalate to release owner with preserved artifacts. |
| Peer root mismatch | Remove peer from authority. | Run federation recovery and replay compare. |
| Upgrade fails | Roll back using pre-upgrade checkpoint and artifacts. | File incident report and block release. |
| Evernode deploy fails | Do not activate runtime. | Verify manifests, hashes, storage, and provider logs. |
