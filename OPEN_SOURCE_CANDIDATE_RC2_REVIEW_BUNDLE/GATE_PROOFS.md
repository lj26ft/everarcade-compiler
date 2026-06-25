# Gate Proofs

## Contributor Gate

Command: `bash scripts/validate_open_source_readiness.sh`

Proves:

- Toolchain reproducibility for the pinned contributor environment.
- Offline vendor restoration and dependency availability.
- Local runtime package generation and onboarding checks.

Does not prove:

- Production deployment.
- World certification.
- Operator trust or public-network availability.

## World Artifact Gate

Command: `EVERARCADE_DETERMINISTIC_ATTEST=1 bash scripts/ci/run-deterministic-world-factory.sh`

Proves:

- Deterministic `world.evr` generation from committed inputs.
- Package verification and payload digest validation.
- Release attestation creation and verification against the committed trust root.
- Deployment bundle generation.

Does not prove:

- Operator trust.
- Production uptime.
- Long-term public-testnet or mainnet service health.

## Attestation Gate

Command: `node creator-sdk/cli/everarcade.mjs world attest verify --project examples/world-factory/frontier-settlement --trusted-public-key "$TRUSTED_PUBLIC_KEY"`

Proves:

- Signed claims are syntactically valid and signature-verified.
- The signer is the trusted review key from `TRUST_ROOT.md`.
- Root equivalence between signed attestation claims and generated release roots.
- Replay equivalence between signed replay claims and replay evidence.

Does not prove:

- The trust root itself is correct beyond the reviewer-selected root document.
- Future key validity after rotation or revocation.
