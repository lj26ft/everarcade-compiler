# EverArcade Verification Story v1

EverArcade verification starts with a World Package and ends with independently repeatable evidence. The v0.1 path is local and reviewable; it is not a production network trust claim.

## Concepts

- **World Contract**: the deterministic rules and metadata that describe what a World is allowed to do.
- **RustRig Invariants**: reusable deterministic invariant modules, such as combat, inventory, market, governance, identity, movement, resources, crafting, structures, quests, continuity, and operations.
- **Certification**: the local process that hashes package artifacts, maps them to proof metadata, and emits a world-package certificate.
- **Replay**: recomputing deterministic execution evidence from recorded inputs and journal material.
- **Restore**: loading checkpoint or package state so an operator can verify continuity after interruption.
- **Migration**: moving a World Package or continuity lineage to a newer package while preserving verifiable roots and compatibility evidence.
- **Independent Verification**: a reviewer recomputes artifact digests and proof links without trusting the creator's local process.

## Artifact chain

```text
World source
↓
World Contract metadata
↓
RustRig invariant selection
↓
World Package (`dist/world.evr` and `dist/runtime-package/`)
↓
Certification artifacts (`dist/certification/`)
↓
Independent proof re-check
↓
Deployment record
```

## Tamper story

```text
Modify artifact
↓
Artifact digest changes
↓
Certificate digest comparison fails
↓
Certification invalidated
↓
Re-certification required before deployment evidence should be trusted
```

For example, changing `dist/runtime-package/manifest.json`, `dist/runtime-package/world.json`, or the packaged wasm after certification causes the independent proof re-check to compare new hashes against the recorded certificate hashes and fail.

## Creator-to-verifier bridge

Creators run:

```bash
node creator-sdk/cli/everarcade.mjs world package --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world verify --project "$PROJECT"
```

Verifiers inspect:

- package manifest digest,
- world metadata digest,
- wasm digest,
- certified kernel compatibility metadata,
- local runtime receipts,
- replay proof reports,
- deployment records that reference a valid certificate.

This bridge connects Creator SDK outputs to Tier-2 verification by making every trusted claim reducible to an artifact path, digest, invariant scope, and repeatable local command.
