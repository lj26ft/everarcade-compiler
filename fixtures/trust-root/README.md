# Test Attester Trust Root Fixture

TEST ONLY. NOT PRODUCTION. NOT EVERARCADE MAINNET TRUST ROOT.

The private key in this directory is committed and therefore PUBLIC. Any
signature it produces is a TEST FIXTURE signature. Pass `--test-fixture-key`
to `world attest create` so the attestation records
`attester.key_class = TEST_FIXTURE_NOT_PRODUCTION`; verification then reports
`PASS_TEST_FIXTURE_NOT_PRODUCTION`, which is distinct both from a production
PASS and from `UNSIGNED` (no signature at all).

The `key_class` label is declared by the signer (opt-in flag). A signer who
omits `--test-fixture-key` gets an unlabelled signature and a plain `PASS`, so
the label is not a security control. Production protection comes from
refusing this key identity outright, not from the label.

Public test key.
Never used for production.
Safe to publish.

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
