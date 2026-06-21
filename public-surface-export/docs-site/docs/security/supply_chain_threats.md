# Supply-Chain Threats

## Scope
Vendor dependencies, lockfiles, and release artifacts.

## Controls
- Pinned `Cargo.lock` and vendored dependencies.
- Deterministic vendor hashing and integrity checks.
- Offline `--locked --frozen` validation pathways.

## Residual Risks
- Compromised upstream before vendoring.
- Maintainer key compromise.
- Build-host toolchain compromise.

## Next Hardening
- Sigstore/cosign attestation.
- Two-person release signing policy.
- Hermetic toolchain image with digest pinning.
