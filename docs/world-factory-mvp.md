# World Factory MVP

The World Factory MVP proves the compiler bridge from planning artifacts to a verifiable `world.evr` package:

```text
Prompt
↓
Genesis
↓
Workspace
↓
Blueprint
↓
Contract Plan
↓
Generated world.evr
↓
Verification
```

This milestone starts with a deterministic Frontier Settlement example. The CLI writes a `WORLD_BLUEPRINT_V1` and `WORLD_CONTRACT_PLAN_V1`, validates their references, generates a minimal `WORLD_EVR_PACKAGE_SPEC_V1` directory package, and verifies it with the independent V1 verifier.

## Commands

```bash
node creator-sdk/cli/everarcade.mjs world factory init
node creator-sdk/cli/everarcade.mjs world factory validate --project examples/world-factory/frontier-settlement
node creator-sdk/cli/everarcade.mjs world factory generate --project examples/world-factory/frontier-settlement
node creator-sdk/cli/everarcade.mjs world factory verify --project examples/world-factory/frontier-settlement
```

## Boundaries

This MVP generates a minimal deterministic package. It does not yet generate Rust gameplay code, deploy to EverNode, or provision leases. It proves the World Factory bridge from blueprint and contract plan into a V1-verifiable package artifact.
