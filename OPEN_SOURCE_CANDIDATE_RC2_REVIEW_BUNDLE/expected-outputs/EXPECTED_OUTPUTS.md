# Expected Outputs

- Contributor Gate: `READY`.
- World Artifact Gate: `PASS` for deterministic generation, verification, attestation, and release bundle checks. In RC2 its attestation check uses the revoked v1 TEST FIXTURE key in non-production reproduction only; that part is not production trust.
- Attestation verification with the key from `TRUST_ROOT.md` (the revoked `everarcade-review-root-v1` TEST FIXTURE key): historical `PASS` in non-production RC2 reproduction only. This is a TEST FIXTURE result from a revoked root, not a production `PASS`. In production the key is refused and verification fails closed.
- Attestation verification with generated fork key trusted by the fixture itself: invalid reviewer procedure; do not accept.
- Self-attested fork verified against `TRUST_ROOT.md`: `FAIL`.
- Tampered payload verified against package hashes or signed roots: `FAIL`.
- Determinism evidence: two isolated generations compare equal for package hash, factory report, deployment manifest, and runtime report.
