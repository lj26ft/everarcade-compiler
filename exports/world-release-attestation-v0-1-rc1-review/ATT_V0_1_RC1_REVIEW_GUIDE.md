# ATT V0.1 RC1 Review Guide

Positive check:

```bash
node creator-sdk/cli/everarcade.mjs world attest verify \
  --attestation exports/world-release-attestation-v0-1-rc1-review/world-release-attestation.json \
  --world exports/world-release-attestation-v0-1-rc1-review/world.evr \
  --trusted-public-key "$(cat exports/world-release-attestation-v0-1-rc1-review/trusted-public-key.txt)"
```

Failure fixtures:

```bash
for fixture in exports/world-release-attestation-v0-1-rc1-review/failure-fixtures/*; do
  echo "== $fixture =="
  if [ -d "$fixture/world.evr" ]; then world="$fixture/world.evr"; else world="exports/world-release-attestation-v0-1-rc1-review/world.evr"; fi
  key="$(cat exports/world-release-attestation-v0-1-rc1-review/trusted-public-key.txt)"
  if [ -f "$fixture/trusted-public-key.txt" ]; then key="$(cat "$fixture/trusted-public-key.txt")"; fi
  if [ "$(basename "$fixture")" = "missing-trusted-key" ]; then
    node creator-sdk/cli/everarcade.mjs world attest verify \
      --attestation "$fixture/world-release-attestation.json" \
      --world "$world" && exit 1 || true
  else
    node creator-sdk/cli/everarcade.mjs world attest verify \
      --attestation "$fixture/world-release-attestation.json" \
      --world "$world" \
      --trusted-public-key "$key" && exit 1 || true
  fi
  cat "$fixture/expected-failure.txt"
done
```
