# Runtime Release Candidate Validation

## Purpose

The runtime release candidate validation gate is the canonical operator entrypoint
for confirming that a Runtime Platform release candidate can be restored from the
published vendor artifact and validated fully offline.

The gate intentionally covers only the current runtime release milestone:

- Vendor artifact restoration
- Offline Cargo metadata resolution
- Offline `everarcade-runtime` compilation
- Offline `everarcade-runtime` test execution
- Release candidate report generation

## Required Artifacts

Before running the gate, the repository must contain the release vendor artifact
and its checksum:

```text
dist/vendor.tar.gz
dist/vendor.tar.gz.sha256
```

These files are generated or copied by the vendor artifact workflow. They are
operator artifacts and must not be committed to source control.

## Validation Workflow

Run the release candidate validation script from the repository root:

```bash
bash scripts/run_release_candidate_validation.sh
```

The script performs the following steps:

1. Verifies `dist/vendor.tar.gz` exists.
2. Verifies `dist/vendor.tar.gz.sha256` exists.
3. Restores `vendor/` with `bash scripts/restore_vendor_artifact.sh`.
4. Runs `bash scripts/check_runtime_offline_gate.sh`.
5. Writes `reports/release_candidate_validation_report.txt`.

The offline gate validates:

```bash
cargo metadata --offline --locked
cargo check -p everarcade-runtime --offline --locked
cargo test -p everarcade-runtime --tests --offline --locked
```

## Expected Success Output

A successful run ends with:

```text
Vendor Restore: PASS
Cargo Metadata: PASS
Runtime Check: PASS
Runtime Tests: PASS
Release Candidate Validation: PASS
```

The report contains:

```text
Timestamp
Vendor Restore Status
Offline Metadata Status
Offline Check Status
Offline Test Status
Overall Result
```

## Failure Conditions

The script fails if any required vendor artifact is missing, checksum verification
fails, `vendor/` cannot be restored, Cargo metadata cannot resolve offline, the
runtime crate does not compile offline, or runtime tests fail offline.

On failure, the report is still written with the status known at the point of
failure.

## Operator Usage

Use this gate after obtaining the vendor artifact for a runtime release
candidate and before promoting that candidate to downstream release automation.

Do not use this gate to rebuild vendor contents, update `Cargo.lock`, certify
Evernode deployment, test upgrades, or perform recovery certification. Those
remain separate release milestones.
