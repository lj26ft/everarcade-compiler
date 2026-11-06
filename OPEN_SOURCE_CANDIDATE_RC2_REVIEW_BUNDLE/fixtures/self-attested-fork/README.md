# Self-Attested Fork Fixture

Expected result: `FAIL`.

Attack simulated: a fork generates a new Ed25519 keypair, signs its own artifacts, and emits its own generated `trusted-public-key.txt`.

Reviewer rule: ignore the generated key and verify with the key extracted from `TRUST_ROOT.md`, which is labelled TEST FIXTURE ONLY (revoked `everarcade-review-root-v1`). This reproduces the RC2 must-fail check only. Production must refuse that key by identity, so production verification fails closed whatever the fork does.

Expected detection: attestation verification fails with an untrusted attester key or signature mismatch. Changing only the signing key must fail review.
