# Frontier Settlement RC1 Public Proof Bundle

This bundle is the public verification handoff for Frontier Settlement RC1.

Files:

- `deployment-manifest.json` — package hash, world hash, continuity root, deployment target, and attestation hash.
- `deployment-report.json` — package, replay, remote, and deployment status roots.
- `live-deployment-proof.json` — live host proof collected from `http://127.0.0.1:8787` during RC1 validation.
- `release-report.json` — release attestation hash and status.
- `world-release-attestation.json` — signed release claims.

Verify with:

```bash
node creator-sdk/cli/everarcade.mjs world factory verify
node creator-sdk/cli/everarcade.mjs world factory replay
node creator-sdk/cli/everarcade.mjs world attest verify --trusted-public-key "$(cat examples/world-factory/frontier-settlement/out/release/trusted-public-key.txt)"
```
