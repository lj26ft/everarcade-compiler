# Clean Clone Runtime Certification

## Purpose

Clean clone runtime certification proves that an EverArcade Runtime release
candidate can be validated from a fresh repository checkout without relying on
any developer-specific machine state.

The certification verifies this release path:

```text
Fresh Clone
  -> Restore Vendor
  -> Validate Runtime
  -> PASS
```

## Required Inputs

The clean checkout must include repository source plus the runtime release vendor
artifact and checksum:

```text
dist/vendor.tar.gz
dist/vendor.tar.gz.sha256
```

The artifact and checksum are release inputs. They are not rebuilt, regenerated,
or modified by this certification.

## Validation Workflow

Run the certification script from the repository root:

```bash
bash scripts/certify_clean_clone_runtime.sh
```

The script performs the following steps:

1. Verifies `dist/vendor.tar.gz` exists.
2. Verifies `dist/vendor.tar.gz.sha256` exists.
3. Deletes any existing `vendor/` directory.
4. Restores `vendor/` with `bash scripts/restore_vendor_artifact.sh`.
5. Runs `bash scripts/run_release_candidate_validation.sh`.
6. Writes `reports/clean_clone_certification_report.txt`.

## PASS Criteria

Certification passes only when all of the following are true:

- The vendor artifact and checksum are present.
- `vendor/` is restored from the artifact successfully.
- Release candidate validation completes successfully.
- The certification report records `Overall Result: PASS`.

A successful run ends with:

```text
Vendor Restore: PASS
Cargo Metadata: PASS
Runtime Check: PASS
Runtime Tests: PASS
Release Candidate Validation: PASS
Clean Clone Certification: PASS
```

## FAIL Criteria

Certification fails if any required artifact is missing, checksum verification
fails, vendor restoration fails, release candidate validation fails, or the final
report cannot be written.

On failure, the report is still written with the status known at the point of
failure.

## Relationship To Release Candidate Validation

Clean clone certification is a portability wrapper around release candidate
validation. Release candidate validation proves the restored runtime can build
and test offline; clean clone certification first removes any existing `vendor/`
directory and restores it from the release artifact to prove the same validation
works without hidden local state.

This milestone does not rebuild vendor contents, update dependencies, modify
runtime code, certify deployment, test upgrades, or validate recovery workflows.
