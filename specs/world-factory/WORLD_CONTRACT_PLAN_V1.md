# WorldContractPlanV1

`WORLD_CONTRACT_PLAN_V1` is the minimal deterministic contract planning document consumed by the World Factory MVP generator.

Required fields:

```json
{
  "schema_version": "WORLD_CONTRACT_PLAN_V1",
  "world_id": "frontier-settlement-demo",
  "contract_version": "0.1.0",
  "planned_mutations": [
    "inventory.transfer",
    "market.trade",
    "governance.vote"
  ],
  "safety_invariants": {
    "inventory.transfer": [
      "ownership",
      "no-overdraw",
      "conservation",
      "atomicity"
    ],
    "market.trade": [
      "ownership",
      "no-double-spend",
      "value-conservation",
      "atomicity"
    ],
    "governance.vote": [
      "eligibility",
      "one-vote-per-member",
      "tally-integrity"
    ]
  },
  "verification_requirements": [
    "package",
    "replay",
    "restore"
  ]
}
```

Every planned mutation must be present in the blueprint capabilities, and every planned mutation must have a non-empty invariant list.
