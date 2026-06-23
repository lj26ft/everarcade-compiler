# Threat Model

## Protected

- Key continuity through `previous_keys`.
- Key revocation through `status = REVOKED`.
- Trusted operator lookup by stable `operator_id`.
- Tamper evidence through deterministic SHA-256 registry hashing.

## Not Protected

- Legal identity.
- Wallet ownership.
- On-chain identity.
- Sybil resistance.
- Permission enforcement.
