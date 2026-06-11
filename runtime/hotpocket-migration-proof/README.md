# EverArcade Sovereign Runtime Migration Proof v0.1

This proof demonstrates that an EverArcade world executed under HotPocket-style three-validator consensus can be exported from one cluster, transferred only as canonical artifacts, restored on an independent cluster, verified for root equivalence, replayed from genesis, and continued deterministically.

## Validation

```bash
node runtime/hotpocket-migration-proof/validation/hotpocket-migration-proof.js validate
```

The validator writes canonical artifacts to `export/`, copies only those artifacts into `import/` for the simulated transfer boundary, writes replay evidence to `replay/`, and emits reports under `reports/`.

## Certification

```bash
bash scripts/certify_hotpocket_migration.sh
```

Expected final line:

```text
EverArcade Sovereign Runtime Migration Proof v0.1: PASS
```

## Scope

The proof is limited to sovereign runtime mobility:

```text
Run → Replay → Verify → Restore → Continue
```

It does not include XRPL anchoring, Xahau settlement, Evernode production leases, federation economics, inventory systems, or renderer projection.
