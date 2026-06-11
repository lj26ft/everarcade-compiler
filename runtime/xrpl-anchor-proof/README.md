# XRPL / Xahau Anchor Publication Proof v0.1

This proof publishes the latest runtime-derived EverArcade continuity anchor into deterministic XRPL and Xahau publication ledgers, retrieves the published payloads, and verifies the retrieved hashes against the Runtime Integration, Migration, Continuity Anchor, Gameplay, and HotPocket anchor acceptance artifacts.

The proof never creates synthetic anchors. It consumes `runtime/continuity-anchor-proof/continuity/continuity-chain.json` and publishes the latest anchor in that chain.

## Layout

```text
runtime/xrpl-anchor-proof/
├── payload_builder/   # canonical anchor payload builder
├── publisher/         # transaction construction and deterministic submission
├── retrieval/         # published payload retrieval
├── verifier/          # hash and dependency verification
├── certification/     # final certification report generation
├── validation/        # proof entrypoint
├── ledgers/           # deterministic XRPL/Xahau publication ledgers
└── reports/           # generated proof reports
```

## Validation

```bash
node runtime/xrpl-anchor-proof/validation/xrpl-anchor-proof.js validate
```

Expected output:

```text
PASS
```

The final certification report ends with:

```text
XRPL / Xahau Anchor Publication Proof v0.1: PASS
```
