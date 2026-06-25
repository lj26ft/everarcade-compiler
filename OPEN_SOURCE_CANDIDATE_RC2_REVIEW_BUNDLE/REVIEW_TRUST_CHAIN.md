# Reviewer Trust Chain

No trust is implicit in Open Source Candidate RC2. Review starts at the trust root and flows only through verified cryptographic checks.

```text
Official Trust Root
  ↓ pins
Review Key
  ↓ verifies signature on
Attestation
  ↓ signs claims about
world.evr
  ↓ binds via manifest/hash-manifest
Payload
  ↓ supplies deterministic inputs to
Replay
  ↓ reproduces behavior expected from
Live Runtime
```

## What the Reviewer Trusts

- The committed `TRUST_ROOT.md` or a separately published byte-equivalent trust-root document.
- The review key named by that trust root.
- The verifier result that the attestation signature matches the review key.
- The package verifier result that `world.evr` matches its manifest and payload digests.
- The replay verifier result that replay evidence is equivalent to the signed roots.

Generated output may be evidence, but it is never the source of reviewer trust.
