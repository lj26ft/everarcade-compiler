# Release Artifact Strategy

## Policy
- Keep source repository lean.
- Generate release tarball artifacts in CI or local release workflows.
- Vendor archive is optional for offline build support.
- ISO image deferred.
- VM image deferred.

## Recommended v0.1 release matrix
- `tar.gz`: **primary** for v0.1.0
- `sha256`: **required** alongside each tarball
- `vendor.tar.gz`: optional offline build support
- container image: v0.1.x track
- Evernode bundle: v0.1.x track
- VM image: later
- ISO: later
