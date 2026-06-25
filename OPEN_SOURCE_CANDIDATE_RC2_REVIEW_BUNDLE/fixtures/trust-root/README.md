# Test Attester Trust Root Fixture

TEST ONLY. NOT PRODUCTION. NOT EVERARCADE MAINNET TRUST ROOT.

These files provide the deterministic attester keypair used by local CI and
Open Source Candidate review gates. They exist so the World Factory Gate can
create an attestation with a committed, out-of-band test key and then verify
that attestation against the matching committed public key.

Review and CI workflows may use this fixture only for test/review attestation
verification:

```bash
node creator-sdk/cli/everarcade.mjs world attest create \
  --attester-private-key fixtures/trust-root/test-attester-private-key.pem
node creator-sdk/cli/everarcade.mjs world attest verify \
  --trusted-public-key "$(cat fixtures/trust-root/test-attester-public-key.txt)"
```

Do not use `out/release/trusted-public-key.txt` as a trust root. Generated
build output is evidence under review, not the source of reviewer trust.
