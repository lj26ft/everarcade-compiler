# ATT V0.1 RC2 Review Guide

## Valid Bundle

This regenerated RC2 bundle is the promoted V0.1 reference artifact. The previous exported RC2 runtime/deployment artifacts were stale and failed current replay re-derivation; use the paths below for freeze verification.

```bash
node creator-sdk/cli/everarcade.mjs world attest verify \
  --attestation exports/world-release-attestation-v0-1-rc2-review/release/world-release-attestation.json \
  --world exports/world-release-attestation-v0-1-rc2-review/world.evr \
  --runtime exports/world-release-attestation-v0-1-rc2-review/runtime \
  --deploy exports/world-release-attestation-v0-1-rc2-review/deploy \
  --trusted-public-key "$(cat exports/world-release-attestation-v0-1-rc2-review/release/trusted-public-key.txt)"
```

## Failure Loop

```bash
for fixture in exports/world-release-attestation-v0-1-rc2-review/failure-fixtures/*; do
  echo "$fixture"
  node creator-sdk/cli/everarcade.mjs world attest verify \
    --attestation "$fixture/release/world-release-attestation.json" \
    --world "$fixture/world.evr" \
    --runtime "$fixture/runtime" \
    --deploy "$fixture/deploy" \
    --trusted-public-key "$(cat "$fixture/release/trusted-public-key.txt")" && exit 1 || true
done
```
