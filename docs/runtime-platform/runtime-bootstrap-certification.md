# Runtime Bootstrap Certification v0.1

## Purpose

Runtime Bootstrap Certification proves that the canonical EverArcade Runtime
Platform v0.1 release artifact bundle can initialize a minimal local runtime
installation without developer intervention.

The certification covers the path from release bundle verification through
vendor restoration, release candidate validation, bootstrap state creation, and a
local health marker.

## Inputs

Required release inputs:

- `dist/everarcade-runtime-v0.1.0-bundle.tar.gz`
- `dist/everarcade-runtime-v0.1.0-bundle.sha256`

The bundle must contain:

- `vendor.tar.gz`
- `vendor.tar.gz.sha256`
- release validation evidence generated during bundle creation

## Bootstrap Workflow

Run:

```bash
bash scripts/bootstrap_runtime_from_bundle.sh
```

The script performs these steps:

1. Verifies that the bundle and checksum files exist.
2. Validates the bundle checksum with `sha256sum -c`.
3. Extracts the bundle into `runtime-bootstrap/`.
4. Restores `vendor/` from the bundled vendor artifact.
5. Runs release candidate validation using the restored vendor tree.
6. Writes `runtime-bootstrap/runtime_bootstrap_report.txt` and a runtime health
   marker.

Then run:

```bash
bash scripts/certify_runtime_bootstrap.sh
```

The certification script validates the generated bootstrap state and writes
`reports/runtime_bootstrap_certification_report.txt`.

## PASS Criteria

Certification passes only when all of the following are true:

- Bundle checksum verification succeeds.
- Bundled vendor artifact and checksum are present after extraction.
- `vendor/` exists after restoration.
- Release candidate validation reports `Overall Result: PASS`.
- Runtime bootstrap report and health marker both exist.
- Runtime bootstrap report records `Runtime Bootstrap: PASS`.

## FAIL Criteria

Certification fails if any required bundle file is missing, checksum validation
fails, bundle extraction fails, vendor restoration fails, release candidate
validation fails, or the runtime bootstrap state is incomplete.

Failure writes the certification report with the failed stage so operators can
identify the first broken handoff.

## Relationship To Release Artifact Bundle

The release artifact bundle is the canonical input. Bootstrap certification does
not create new runtime capabilities, modify protocol behavior, configure
networking, or deploy to Evernode.

It proves that a fresh machine with the bundle can restore dependencies and reach
a validated local runtime bootstrap state before later deployment milestones.
