# RC2 Reviewer Checklist

- [ ] Start from a fresh clone at pinned commit `fe51c1ce5be6df888dfaae203d5632580a045f2e`.
- [ ] Read `TRUST_ROOT.md` and copy the trusted key only from that document or an independently published equivalent.
- [ ] Restore vendor dependencies with `bash scripts/ensure_vendor_offline.sh`.
- [ ] Verify toolchain prerequisites with `bash scripts/check_prerequisites.sh`.
- [ ] Run Contributor Gate: `bash scripts/validate_open_source_readiness.sh`.
- [ ] Run World Artifact Gate: `EVERARCADE_DETERMINISTIC_ATTEST=1 CARGO_BUILD_JOBS=1 bash scripts/ci/run-deterministic-world-factory.sh`.
- [ ] Verify `world.evr` package digests.
- [ ] Verify attestation using the trusted key from `TRUST_ROOT.md`, not generated output.
- [ ] Run `fixtures/self-attested-fork/` and confirm `FAIL`.
- [ ] Run `fixtures/tampered-reference-world/` and confirm `FAIL`.
- [ ] Confirm determinism evidence includes two isolated generations and output comparisons.
