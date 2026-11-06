# Frontier Settlement Fixture Keys

The key material in this directory is a public test key. It is committed,
so it is PUBLIC and must be treated as compromised for any non-test purpose.
Signatures made with it are TEST FIXTURE signatures
(`TEST_FIXTURE_NOT_PRODUCTION`), never production attestations. The
`key_class` label is signer-declared (opt-in flag), so it is not a security
control; production protection comes from refusing the key identity.

- Public test key.
- Never used for production.
- Safe to publish.

It intentionally matches the trust-root test attester fixture so the World Factory and attestation review paths can be reproduced from a clean clone. Do not use this key for real operators, production signing, settlement, wallets, or deployment authority.
