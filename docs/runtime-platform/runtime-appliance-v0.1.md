# EverArcade Runtime Appliance v0.1

## Purpose

The EverArcade Runtime Appliance v0.1 is the first deployable runtime distribution for a fresh Ubuntu VM. It packages the certified runtime surface into an installable, checksummed bundle that can be installed, validated, operated, replayed, and checkpointed without requiring `cargo`, `rustup`, `git`, source code, or a development environment on the target machine.

This milestone is packaging and operationalization only. It does not certify a new protocol, add a runtime subsystem, or introduce HotPocket, Evernode, XRPL RPC, GPU runtime, renderer runtime, or protocol changes.

## Appliance Layout

Canonical bundle root:

```text
everarcade-runtime-v0.1/
  bin/everarcade-runtime
  runtime/
    config/
    worlds/
      civilization-alpha/
        package/
        journals/
        checkpoints/
        backups/
        receipts/
        state/
    journals/
    checkpoints/
    backups/
    reports/
    vendor/
  scripts/
    install.sh
    validate.sh
    doctor.sh
    start.sh
    stop.sh
    status.sh
  docs/runtime-appliance-v0.1.md
  APPLIANCE_LAYOUT.txt
  MANIFEST.sha256
```

`bin/everarcade-runtime` is the stable runtime entry point. The scripts directory provides operator-safe wrappers. `MANIFEST.sha256` covers all files in the extracted appliance.

## Installation

From a repository or release workspace that contains the bundle:

```bash
bash scripts/build_runtime_appliance.sh
bash scripts/install_runtime_appliance.sh
```

The default install prefix is `appliance/installed`. Operators may override it:

```bash
EVERARCADE_APPLIANCE_PREFIX=/opt/everarcade-runtime \
  bash scripts/install_runtime_appliance.sh dist/everarcade-runtime-v0.1.tar.gz /opt/everarcade-runtime
```

Inside an extracted appliance, run:

```bash
bash scripts/install.sh
```

The installer fails closed if the bundle checksum, appliance manifest, runtime binary, configuration, or package manifest is missing or invalid.

## Validation

Run:

```bash
bash scripts/validate_runtime_appliance.sh
```

Inside an extracted appliance:

```bash
bash scripts/validate.sh
```

Validation checks vendor artifact presence, release-candidate layout readiness, protocol sovereignty evidence, runtime start, replay verification, checkpoint creation, and stop.

## Doctor

Run:

```bash
bash scripts/runtime_doctor.sh
```

Inside an extracted appliance:

```bash
bash scripts/doctor.sh
```

Doctor verifies the directory layout, configuration files, checkpoints area, journals area, reports area, vendor artifact marker, package manifest, and runtime doctor command. It prints `PASS` on success or `FAIL` with actionable errors.

## Operations

Repository wrappers target the installed appliance:

```bash
bash scripts/runtime_start.sh
bash scripts/runtime_status.sh
bash scripts/runtime_stop.sh
```

Inside an extracted appliance, use:

```bash
bash scripts/start.sh
bash scripts/status.sh
bash scripts/stop.sh
```

The wrappers resolve the appliance root, world id, runtime data root, and package path before invoking `bin/everarcade-runtime`.

## Reports

Validation writes:

```text
reports/runtime_appliance_validation_report.txt
```

Certification writes:

```text
reports/runtime_appliance_certification_report.txt
```

When validation runs inside the appliance, reports are written under `runtime/reports/`.

## PASS Criteria

Runtime Appliance v0.1 passes when:

1. `dist/everarcade-runtime-v0.1.tar.gz` exists.
2. `dist/everarcade-runtime-v0.1.tar.gz.sha256` verifies.
3. Installation succeeds and verifies `MANIFEST.sha256`.
4. Validation reports `Runtime Appliance Validation: PASS`.
5. Doctor reports `PASS`.
6. Operator start, status, stop, replay, and checkpoint paths are available.
7. Certification reports `Runtime Appliance v0.1: PASS`.

## FAIL Criteria

The appliance fails closed when any required bundle, checksum, manifest, binary, config, package, vendor marker, directory, validation step, doctor step, or certification step is missing or invalid.

## Relationship To Protocol Sovereignty Certification

Protocol Sovereignty Certification remains the authority for protocol correctness. The appliance validation consumes that certification as evidence and proves that the already-certified protocol stack can be packaged and operated as a deployable runtime unit. This milestone does not replace or expand protocol certification.

## Relationship To Future Protocol Node Appliance

This appliance is the foundation for a future Protocol Node Appliance. Later milestones can layer node networking, Evernode integration, HotPocket integration, XRPL connectivity, renderer services, and GPU-backed runtime domains on top of the installation, validation, doctor, operations, manifest, and checksum model established here.
