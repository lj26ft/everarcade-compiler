# WorldBlueprintV1

`WORLD_BLUEPRINT_V1` is the minimal portal-to-compiler blueprint contract for the World Factory MVP.

Required fields:

```json
{
  "schema_version": "WORLD_BLUEPRINT_V1",
  "world_id": "frontier-settlement-demo",
  "world_name": "Frontier Settlement Demo",
  "world_type": "frontier-settlement",
  "governance": "council",
  "economy": "marketplace",
  "capabilities": [
    "inventory.transfer",
    "market.trade",
    "governance.vote"
  ],
  "runtime_profile": "small",
  "verification_targets": [
    "package",
    "replay",
    "restore",
    "migration"
  ],
  "infrastructure_profile": "single-evernode-lease"
}
```

The MVP compiler accepts only `world_type: frontier-settlement`, `runtime_profile: small`, and the listed capability identifiers. The infrastructure profile is declarative only; no EverNode lease is provisioned.
