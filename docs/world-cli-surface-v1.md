# World CLI Surface v1

This document defines the future high-level EverArcade world CLI. These commands are specification only; current equivalents are documented in `docs/canonical-world-creation-flow-v1.md`.

## `everarcade world init`

Creates a world workspace from a template.

```bash
everarcade world init --template frontier --name my-frontier
```

Expected behavior:

- Copy template files into a new world directory.
- Generate default metadata, contract, genesis, continuity, projection, registry, and proof mapping files.
- Print next steps for customization and package verification.

## `everarcade world contract`

Inspects, edits, or validates the world contract.

```bash
everarcade world contract validate
everarcade world contract explain
```

Expected behavior:

- Validate contract schema.
- Explain declared capabilities and invariants in developer language.
- Report missing RustRig capability mappings.

## `everarcade world package`

Builds a `.evr` world package.

```bash
everarcade world package
```

Expected behavior:

- Validate required files.
- Create `dist/world.evr`.
- Create a package manifest and content digest.
- Refuse to package unresolved candidate capabilities unless explicitly allowed.

## `everarcade world verify`

Verifies a world package and source workspace.

```bash
everarcade world verify
```

Expected behavior:

- Verify package structure.
- Verify contract, genesis, continuity, projection, registry, and proof mapping files.
- Print human-readable pass/fail results.
- Emit machine-readable verification evidence.

## `everarcade world project`

Launches the projection runtime for a local world.

```bash
everarcade world project
```

Expected behavior:

- Load world state from genesis or package.
- Start a read-only projection view.
- Show world state, replay status, and continuity status.
- Avoid exposing replay implementation details during first-run onboarding.

## `everarcade world deploy`

Prepares or submits a verified world for operator deployment.

```bash
everarcade world deploy --operator local
```

Expected behavior:

- Require a verified package.
- Generate an operator handoff bundle.
- Include registry entry and verification evidence.
- Print deployment status and next operator action.

## `everarcade world replay`

Runs or inspects replay for a world package.

```bash
everarcade world replay --from genesis
```

Expected behavior:

- Replay accepted receipts/events.
- Report deterministic state checkpoints.
- Present replay results as developer-facing health output.

## `everarcade world restore`

Restores world state from a checkpoint or continuity bundle.

```bash
everarcade world restore --checkpoint latest
```

Expected behavior:

- Validate checkpoint provenance.
- Restore world state into a local workspace.
- Report restored world identity, height, and continuity status.

## `everarcade world migrate`

Migrates a world between contract or template versions.

```bash
everarcade world migrate --to contract-v2
```

Expected behavior:

- Show planned file changes before writing.
- Preserve world identity and genesis provenance.
- Require verification after migration.
