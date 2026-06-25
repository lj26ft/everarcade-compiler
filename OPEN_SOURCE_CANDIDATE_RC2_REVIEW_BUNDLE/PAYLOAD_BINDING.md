# Payload Binding

The Open Source Candidate RC2 artifact chain is transitive. A reviewer verifies `world.evr` by checking every digest edge below, not by trusting filenames or generated summaries.

```text
world.evr
  ↓ contains or references
manifest.json
  ↓ records digest of
hash-manifest.json
  ↓ records SHA-256 file digests for
payload files
  ↓ bind
blueprint
contract-plan
runtime payload
journal / replay evidence
deployment payload
```

## Hash Relationships

1. `world.evr` is the canonical packaged world artifact produced by the World Artifact Gate.
2. `manifest.json` is the package manifest inside the world artifact. Its own bytes are covered by the package digest.
3. `manifest_sha256` is the SHA-256 digest of the canonical `manifest.json` bytes.
4. `manifest.json` names `hash-manifest.json` and records the expected SHA-256 digest for it.
5. `hash-manifest.json` is the payload digest table. It records the SHA-256 digest for each payload file that participates in replay, runtime execution, deployment, or certification.
6. Payload file digests bind the blueprint, contract-plan, runtime payload, replay journal, and deployment payload. If any payload byte changes and the digest table is not updated, verification fails.
7. The attestation signs claims over the package root, `manifest_sha256`, replay root, and deployment/root equivalence claims. The signature therefore covers the transitive payload binding through the manifest chain.

## Tamper Rule

Modifying any blueprint, contract-plan, runtime, replay/journal, or deployment payload without updating the corresponding hash and re-signing with the trusted review key must produce verification `FAIL`.
