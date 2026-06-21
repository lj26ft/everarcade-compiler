# Vendor Artifact Certification

## Purpose

Vendor artifact certification verifies that the checked release artifact can restore a
`vendor/` tree that is complete enough for offline Cargo dependency resolution.

This certification does not rebuild, regenerate, or repair vendor content. It only
answers whether the existing `dist/vendor.tar.gz` artifact is suitable for offline
runtime validation.

## Validation Workflow

Run:

```bash
bash scripts/certify_vendor_artifact.sh
```

The script performs these checks:

1. Confirms `dist/vendor.tar.gz` exists.
2. Confirms `dist/vendor.tar.gz.sha256` exists.
3. Restores `vendor/` with `bash scripts/restore_vendor_artifact.sh`.
4. Runs `cargo metadata --offline --locked --format-version 1`.
5. Writes `reports/vendor_artifact_certification_report.txt`.

## PASS Criteria

Certification passes only when all of the following are true:

- The artifact and checksum files are present.
- The restore script succeeds, including checksum validation.
- Cargo metadata resolves successfully with `--offline --locked`.

A PASS result means the restored artifact contains the dependency sources needed
for offline dependency resolution.

## FAIL Criteria

Certification fails when any required artifact file is missing, restore fails, or
Cargo cannot resolve dependencies offline after restore.

A FAIL result means the artifact is not complete enough for offline runtime
validation. The certification script does not attempt to repair that condition.

## Relationship to Release Candidate Validation

Release candidate validation performs broader runtime checks after restoring the
vendor artifact. Vendor certification is narrower: it certifies artifact
completeness for offline dependency resolution before the release candidate gate
continues into runtime check and test steps.
