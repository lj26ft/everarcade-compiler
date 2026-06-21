# Evernode-Compatible Deployment Layer v0.1

## Purpose

Evernode-Compatible Deployment Layer v0.1 proves that an EverArcade Protocol Node can be packaged, installed, started, stopped, diagnosed, recovered, and upgrade-validated inside an Evernode-compatible deployment boundary.

Evernode remains infrastructure: it hosts a runtime. EverArcade remains authoritative for protocol state, replay roots, continuity roots, settlement roots, and authority roots.

This milestone does not perform live Evernode registration, public lease deployment, networking, consensus, XRPL RPC, Hooks, Xaman signing, renderer runtime, or GPU runtime work.

## Phase 1 Scope

Phase 1 established the canonical `evernode/` layout, bootstrap, validation, deployment certification, and reports through the existing Evernode Phase 1 scripts.

## Phase 2 Scope

Phase 2 adds operational lifecycle management: package build, install, start, health, recovery, upgrade, stop, and operations certification.

The target stack is:

```text
Protocol Node
        ↓
HotPocket Adapter
        ↓
Evernode Deployment Package
        ↓
Install / Start / Checkpoint / Stop / Recover / Upgrade
```

## Deployment Layout

```text
evernode/
  lease/        Deployment manifest and lease metadata placeholder.
  runtime/      Runtime deployment assets placeholder.
  node/         Protocol Node Appliance deployment bindings.
  hotpocket/    HotPocket deployment bindings.
  checkpoints/  Deployment checkpoint exports.
  journals/     Deployment journal exports.
  backups/      Backup and upgrade validation artifacts.
  logs/         Deployment logs.
  reports/      Deployment-local report copies.
  install/      Generated package installation target.
```

Generated runtime artifacts and package outputs remain ignored; source-controlled `.gitkeep` files preserve empty directories.

## Package Build

Run:

```bash
bash scripts/build_evernode_package.sh
```

The builder creates:

```text
dist/everarcade-evernode-v0.1.tar.gz
dist/everarcade-evernode-v0.1.tar.gz.sha256
reports/evernode_package_build_report.txt
```

The package contains the Evernode layout, runtime appliance assets, Protocol Node assets, HotPocket assets, operation scripts, and this documentation. Package artifacts are generated deployment outputs and are not committed.

Expected output:

```text
Package Build: PASS
```

## Installation

Run:

```bash
bash scripts/install_evernode_package.sh
```

Installation verifies:

- SHA-256 checksum.
- Package manifest.
- Required package layout.

It installs the verified package under `evernode/install/current` and writes `reports/evernode_install_report.txt`. The installer fails closed if checksum, manifest, or layout verification fails.

Expected output:

```text
Installation: PASS
```

## Start

Run:

```bash
bash scripts/evernode_start.sh
```

Start verifies runtime, Protocol Node, and HotPocket assets. It initializes the Protocol Node Appliance, starts it, creates a checkpoint, initializes the HotPocket adapter layout, exports a checkpoint envelope, mirrors checkpoint and journal pointers into `evernode/`, and writes `reports/evernode_start_report.txt`.

Expected output:

```text
Start: PASS
```

## Health

Run:

```bash
bash scripts/evernode_health.sh
```

Health verifies:

- Runtime layout.
- Node layout.
- Checkpoint presence.
- Report presence.
- HotPocket presence.

It writes `reports/evernode_health_report.txt`.

Expected output:

```text
Health: PASS
```

## Recovery

Run:

```bash
bash scripts/evernode_recover.sh
```

Recovery verifies that a checkpoint exists and contains the expected checkpoint fields. It then uses existing Protocol Node replay and restore scripts to verify replay, restore world state, preserve journals, and write the latest continuity root.

Recovery must preserve deterministic state: a valid checkpoint plus journal must reproduce the same continuity root for the same input state. Recovery writes `reports/evernode_recovery_report.txt`.

Expected output:

```text
Recovery: PASS
```

## Upgrade

Run:

```bash
bash scripts/evernode_upgrade.sh
```

Upgrade validation performs no version migration. It verifies that:

- A backup is created.
- The package checksum is valid.
- Evernode layout is preserved.
- The latest checkpoint hash is preserved.

It writes `reports/evernode_upgrade_report.txt`.

Expected output:

```text
Upgrade: PASS
```

## Stop

Run:

```bash
bash scripts/evernode_stop.sh
```

Stop delegates to the Protocol Node stop flow, verifies persisted runtime state, verifies journal presence, mirrors the journal into `evernode/journals/`, and writes `reports/evernode_stop_report.txt`.

Expected output:

```text
Stop: PASS
```

## Operations Certification

Run:

```bash
bash scripts/certify_evernode_operations.sh
```

Certification runs `Package Build → Install → Start → Health → Recovery → Upgrade → Stop` and writes `reports/evernode_operations_certification_report.txt`.

Expected output:

```text
Evernode Operations: PASS
```

## PASS Criteria

Phase 2 passes when package build, checksum creation, installation verification, runtime start, health diagnostics, recovery, upgrade validation, stop, and operations certification all report `PASS`.

## FAIL Criteria

Phase 2 fails if required assets are missing, checksum or manifest verification fails, start cannot initialize/checkpoint, health cannot find required assets, recovery cannot verify checkpoint/replay/restore/continuity output, upgrade cannot create a backup or preserve checkpoint identity, stop cannot persist state or journals, or any non-goal is introduced.

## Relationship To Future Live Lease Deployment

Phase 2 is deployment-readiness only. It proves that the package and lifecycle boundaries are operational before adding live Evernode registration or public lease deployment. Future live deployment work may bind this package to Evernode lease mechanics, but must keep EverArcade protocol state authoritative and must not mutate replay, continuity, settlement, or authority roots through infrastructure operations.
