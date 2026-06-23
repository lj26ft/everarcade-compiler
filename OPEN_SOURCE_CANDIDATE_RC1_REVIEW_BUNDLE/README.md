# Open Source Candidate RC1 Review Bundle

Review question: can a stranger reproduce EverArcade without maintainer assistance?

## Required order

1. Restore vendor: `bash scripts/ensure_vendor_offline.sh`
2. Run prerequisites: `bash scripts/check_prerequisites.sh`
3. Run Contributor Gate: `bash scripts/validate_open_source_readiness.sh`
4. Run World Artifact Gate: `EVERARCADE_DETERMINISTIC_ATTEST=1 bash scripts/ci/run-deterministic-world-factory.sh`
5. Verify attestation with the generated trusted key.

## Gate boundaries

Contributor Gate produces a local runtime/development proof and readiness report. World Artifact Gate produces and verifies `world.evr`, replay evidence, deployment bundle, and attestation.
