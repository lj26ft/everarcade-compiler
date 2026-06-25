# Expected Outputs

- Contributor Gate: `READY`.
- World Artifact Gate: `PASS` for deterministic generation, verification, attestation, and release bundle checks.
- Attestation verification with key from `TRUST_ROOT.md`: `PASS`.
- Attestation verification with generated fork key trusted by the fixture itself: invalid reviewer procedure; do not accept.
- Self-attested fork verified against `TRUST_ROOT.md`: `FAIL`.
- Tampered payload verified against package hashes or signed roots: `FAIL`.
- Determinism evidence: two isolated generations compare equal for package hash, factory report, deployment manifest, and runtime report.
