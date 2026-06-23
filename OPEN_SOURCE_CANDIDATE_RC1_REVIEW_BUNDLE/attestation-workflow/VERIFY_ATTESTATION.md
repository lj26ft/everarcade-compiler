# Attestation workflow

After the world artifact gate, run:

```bash
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --trusted-public-key "$(cat examples/world-factory/frontier-settlement/out/release/trusted-public-key.txt)"
```
