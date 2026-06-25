# Attestation workflow

After the world artifact gate, run:

```bash
node creator-sdk/cli/everarcade.mjs world attest verify \
  --project examples/world-factory/frontier-settlement \
  --trusted-public-key "$(awk '/^```text$/{block++; next} block==1 && /^[A-Za-z0-9+\/=]+$/{print; exit}' TRUST_ROOT.md)"
```
