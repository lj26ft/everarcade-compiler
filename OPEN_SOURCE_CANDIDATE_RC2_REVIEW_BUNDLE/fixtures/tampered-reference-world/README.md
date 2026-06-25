# Tampered Reference World Fixture

Expected result: `FAIL`.

Attack simulated: a replay, blueprint, or runtime payload is modified after `world.evr` generation without updating `hash-manifest.json`, `manifest.json`, and the signed attestation.

Reviewer rule: run package verification and attestation verification after tampering.

Expected detection: package digest verification or attestation root-equivalence verification fails. Any payload byte change without matching hash updates and trusted re-signing must fail review.
