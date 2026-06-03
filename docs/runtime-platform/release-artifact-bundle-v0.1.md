# EverArcade Runtime Platform v0.1 Release Artifact Bundle

## Purpose

The v0.1 release artifact bundle is the portable handoff package for proving that
a fresh VM can clone this repository, restore the vendored Rust dependencies, and
run release-candidate validation offline.

It is not an Evernode deployment package. It only packages the artifacts needed
to prove this flow:

```text
Fresh VM -> Clone repo -> Restore vendor artifact -> Run validation -> PASS
```

## Bundle Contents

The generated bundle is:

```text
dist/everarcade-runtime-v0.1.0-bundle.tar.gz
dist/everarcade-runtime-v0.1.0-bundle.sha256
```

Inside the tarball:

```text
everarcade-runtime-v0.1.0-bundle/
  vendor.tar.gz
  vendor.tar.gz.sha256
  vendor_artifact_certification_report.txt
  runtime_offline_gate_report.txt
  release_candidate_validation_report.txt
  MANIFEST.txt
```

`MANIFEST.txt` records the bundle name, creation time, vendor SHA256,
certification status, and release-candidate status.

## Build the Bundle

Prerequisites:

```text
dist/vendor.tar.gz
dist/vendor.tar.gz.sha256
```

Build with:

```bash
bash scripts/build_release_artifact_bundle.sh
```

The build script certifies the vendor artifact, runs release-candidate
validation, stages the bundle contents, writes the manifest, creates the bundle
tarball, and writes its SHA256 checksum.

Expected result:

```text
Vendor Artifact Certification: PASS
Release Candidate Validation: PASS
Release Artifact Bundle: PASS
```

## Verify the Bundle

Verify with:

```bash
bash scripts/verify_release_artifact_bundle.sh
```

The verification script checks the bundle checksum, extracts the tarball to:

```text
/tmp/everarcade-runtime-v0.1.0-bundle-verify
```

It then confirms the extracted bundle contains `vendor.tar.gz`,
`vendor.tar.gz.sha256`, and `MANIFEST.txt`, and verifies the bundled vendor
checksum.

Expected result:

```text
Release Artifact Bundle: PASS
```

## Fresh VM Usage

On a fresh VM:

1. Clone the repository.
2. Copy the release bundle and checksum into `dist/`.
3. Run `bash scripts/verify_release_artifact_bundle.sh`.
4. Extract `vendor.tar.gz` from the verified bundle into `dist/`.
5. Run `bash scripts/restore_vendor_artifact.sh`.
6. Run `bash scripts/run_release_candidate_validation.sh`.

A passing run proves the VM can restore vendored dependencies and validate the
runtime offline from the release artifacts.

## Intentionally Not Included

The bundle intentionally does not include:

- `vendor/` as an expanded directory
- runtime source changes
- `Cargo.lock` updates
- Evernode deployment assets
- systemd services
- protocol networking, XRPL, or Xahau integration
- release signatures

## Why Generated Artifacts Are Not Committed

`dist/*.tar.gz`, `dist/*.sha256`, expanded staging directories, and validation
reports are generated release outputs. They can be large, machine-specific, or
regenerated from the canonical repository plus the release vendor artifact.

Attach the bundle tarball and checksum to the release instead of committing them
to git.
