# Fresh Ubuntu VM Runtime Certification

## Purpose

Fresh Ubuntu VM runtime certification proves that an EverArcade Runtime release
candidate can be validated on a machine that has never held project build state.
It certifies this deployment-grade path:

```text
Fresh Ubuntu VM
  -> Clone Repository
  -> Restore Vendor Artifact
  -> Release Candidate Validation
  -> PASS
```

The certification is intended to expose hidden reliance on a developer Cargo
cache, a pre-existing `vendor/` directory, or local workstation history.

## VM Requirements

- Ubuntu with Bash, Git, tar, sha256sum, and the Rust/Cargo toolchain required
  by the repository.
- A clean repository checkout of the release candidate.
- No requirement for a prior Cargo cache, prior `vendor/`, or previous local
  build history.

## Required Artifacts

The VM checkout must receive the release vendor artifact and checksum:

```text
dist/vendor.tar.gz
dist/vendor.tar.gz.sha256
```

These are release inputs. Fresh VM certification does not rebuild vendor
contents, regenerate checksums, update dependencies, or modify `Cargo.lock`.

## Validation Workflow

Run from the repository root:

```bash
bash scripts/certify_fresh_vm_runtime.sh
```

The script:

1. Verifies `dist/vendor.tar.gz` exists.
2. Verifies `dist/vendor.tar.gz.sha256` exists.
3. Removes any existing `vendor/` directory.
4. Restores `vendor/` with `bash scripts/restore_vendor_artifact.sh`.
5. Runs `bash scripts/run_release_candidate_validation.sh`.
6. Writes `reports/fresh_vm_certification_report.txt`.

## PASS Criteria

Certification passes only when all of the following are true:

- The vendor artifact and checksum are present.
- `vendor/` restores successfully from the artifact.
- Release candidate validation reports PASS for vendor restore, Cargo metadata,
  runtime check, and runtime tests.
- `reports/fresh_vm_certification_report.txt` records `Overall Result: PASS`.

A successful run ends with:

```text
Vendor Restore: PASS
Cargo Metadata: PASS
Runtime Check: PASS
Runtime Tests: PASS
Release Candidate Validation: PASS
Fresh VM Certification: PASS
```

## FAIL Criteria

Certification fails if a required artifact is missing, checksum verification
fails, vendor restoration fails, release candidate validation fails, or the
certification report cannot be written.

On failure, the report is still written with the Ubuntu version and the latest
known statuses.

## Relationship To Clean Clone Certification

Clean clone certification proves reproducibility from a fresh checkout in the
current environment. Fresh VM certification extends that proof to an Ubuntu VM
with no prior project state, making the release artifact restoration and runtime
validation path suitable for first-time deployment certification.

This milestone does not deploy to Evernode, add signing, certify recovery,
certify upgrades, change runtime source, rebuild vendor, or update dependencies.
