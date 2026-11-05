# RC2 Reviewer Checklist

- [ ] Start from a fresh clone at RC2 reviewer-entry commit `53a17567e826c5d4f9b083e490cf1568bfe7534e`.
- [ ] Read `TRUST_ROOT.md`. This v1 trust-root document is **REVOKED** (`REVOKED_COMPROMISED_TEST_FIXTURE`), and its key is the **TEST FIXTURE only**. It is not a trusted review key; do not copy it as one. There is no trusted review key at present: `trust-roots/everarcade-review-root-v2.md` has status `PREPARED_NO_ACTIVE_KEY`. When an ACTIVE trust-root version exists, copy the review key only from that ACTIVE trust-root document as committed in this repository, or from a separately published copy of that exact document. Never copy a key from generated build output, release output, a fixture under test, or any artifact whose authenticity is being evaluated.
- [ ] Restore vendor dependencies with `bash scripts/ensure_vendor_offline.sh`.
- [ ] Verify toolchain prerequisites with `bash scripts/check_prerequisites.sh`.
- [ ] Run Contributor Gate: `bash scripts/validate_open_source_readiness.sh`.
- [ ] Run World Artifact Gate: `EVERARCADE_DETERMINISTIC_ATTEST=1 CARGO_BUILD_JOBS=1 bash scripts/ci/run-deterministic-world-factory.sh`.
- [ ] Verify `world.evr` package digests.
- [ ] Verify attestation only against a review key copied from an ACTIVE trust-root document (see above), never against generated output. The key in `TRUST_ROOT.md` is the revoked v1 TEST FIXTURE key: a result against it is a TEST FIXTURE result, not trust, and production verification refuses it. Until a v2 trust root is ACTIVE, no attestation result is trust-root-backed.
- [ ] Run `fixtures/self-attested-fork/` and confirm `FAIL`.
- [ ] Run `fixtures/tampered-reference-world/` and confirm `FAIL`.
- [ ] Confirm determinism evidence includes two isolated generations and output comparisons.
