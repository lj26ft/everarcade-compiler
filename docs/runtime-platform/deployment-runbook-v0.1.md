# EverArcade v0.1 Deployment Runbook

## Purpose
Deploy the v0.1 release candidate from source, artifacts, and the vendor bundle without introducing new runtime features.

## Preconditions
- Confirm `release/manifest/RELEASE_MANIFEST_ROOT` exists.
- Confirm `release/interface-freeze/INTERFACE_FREEZE_ROOT` exists.
- Confirm distribution artifacts listed in `release/distribution-certification/distribution-matrix.tsv` are available.

## Procedure
1. Restore or verify the vendor bundle.
2. Install the runtime package with the frozen CLI surface.
3. Bootstrap the runtime layout.
4. Deploy the node or Evernode package.
5. Publish portal artifacts for external onboarding.
6. Record deployment root, settlement root, and replay root.

## PASS Criteria
- Deployment package installs cleanly.
- Runtime status reports healthy.
- Replay records are available after deployment.
- Distribution certification remains `PASS`.

## FAIL Criteria
- Missing artifact, checksum, or frozen interface file.
- Deployment requires an undocumented runtime capability.
- Replay or settlement evidence cannot be reproduced.
