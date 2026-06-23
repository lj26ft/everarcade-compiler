# Operator Identity Registry RC1

## Purpose

Operator Identity Registry RC1 is the EverArcade trust root for trusted cryptographic identity, key continuity, key rotation, and key revocation for World Operators, Attesters, Verifiers, future Treasury Operators, and future Archive Operators.

It does **not** establish legal identity, wallet ownership, on-chain identity, or Sybil resistance.

## Operator identity record format

```json
{
  "schema_version": "OPERATOR_IDENTITY_RC1",
  "operator_id": "operator.frontier.settlement",
  "display_name": "Frontier Settlement Operator",
  "role": "world_operator",
  "public_key": "...",
  "status": "ACTIVE",
  "created_at": "...",
  "updated_at": "...",
  "previous_keys": []
}
```

Fields:

- `schema_version`: must be `OPERATOR_IDENTITY_RC1`.
- `operator_id`: stable EverArcade operator identifier. It is not a legal identity claim.
- `display_name`: human-readable label.
- `role`: one of `world_operator`, `attester`, `verifier`, `treasury_operator`, or `archive_operator`.
- `public_key`: currently trusted active public key.
- `status`: `ACTIVE` or `REVOKED`.
- `created_at`: record creation timestamp.
- `updated_at`: last registry update timestamp for this record.
- `previous_keys`: append-only key continuity history populated during key rotation.

Roles are recorded only. RC1 does not implement permissions.

## Registry format

```json
{
  "schema_version": "OPERATOR_REGISTRY_RC1",
  "registry_hash": "...",
  "operators": []
}
```

`registry_hash` is a deterministic SHA-256 hash derived from the canonical operator records. The hash exists for tamper evidence and excludes the `registry_hash` field itself.

## Deterministic registry hash

To compute `registry_hash`:

1. Read `operators`.
2. Sort records by `operator_id`, then `public_key`.
3. Serialize the sorted records as canonical compact JSON.
4. Compute SHA-256 over those bytes.
5. Store the lowercase hex digest in `registry_hash`.

## Registration flow

1. Initialize a registry with `everarcade operator registry init` if one does not exist.
2. Submit an operator identity record using `everarcade operator register`.
3. Validate schema, role, status, non-empty `operator_id`, and non-empty `public_key`.
4. Reject duplicate `operator_id` values.
5. Insert the record and recompute `registry_hash`.

## Key rotation flow

1. Locate an `ACTIVE` operator by `operator_id`.
2. Reject rotation for revoked operators.
3. Reject replacement keys that match the current key or already appear in `previous_keys`.
4. Append the current `public_key` to `previous_keys`.
5. Install the replacement key as `public_key`.
6. Update `updated_at` and recompute `registry_hash`.

## Revocation flow

1. Locate the operator by `operator_id`.
2. Set `status` to `REVOKED`.
3. Preserve current and previous keys for audit continuity.
4. Update `updated_at` and recompute `registry_hash`.

## Operator verification flow

`everarcade operator verify` checks:

1. The registry hash is valid.
2. The operator exists.
3. The operator status is `ACTIVE`.
4. The supplied public key matches the record's active `public_key`.

## Registry verification flow

`everarcade operator registry verify` checks:

- registry schema
- deterministic registry hash
- duplicate operator IDs
- duplicate active keys
- invalid roles
- invalid statuses
- broken rotation chains where the active key appears in `previous_keys`

## Attestation integration hooks

RC1 does not modify attestation verification. It defines the future integration boundary:

```text
Operator Registry
↓
Trusted Key Lookup
↓
Attestation Verification
```

Future attestation verification should resolve an attestation's claimed operator identity to the current trusted public key before signature verification.

## Proof boundaries

Protected:

- key continuity
- key revocation
- trusted operator lookup
- tamper evidence for registry contents

Not protected:

- legal identity
- wallet ownership
- on-chain identity
- Sybil resistance
- authorization or role permissions

## Future layers

- RC2: Operator Registry + Attestation Integration
- RC3: XRPL/Xahau Anchoring
- RC4: Xaman Signing
- RC5: World Ownership Registry
