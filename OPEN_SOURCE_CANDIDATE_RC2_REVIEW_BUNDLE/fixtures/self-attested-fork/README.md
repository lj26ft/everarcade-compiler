# Self-Attested Fork Fixture

Expected result: `FAIL`.

Attack simulated: a fork generates a new Ed25519 keypair, signs its own artifacts, and emits its own generated `trusted-public-key.txt`.

Reviewer rule: ignore the generated key and verify with the official key from `TRUST_ROOT.md`.

Expected detection: attestation verification fails with an untrusted attester key or signature mismatch. Changing only the signing key must fail review.
