# Canonical World Creation Flow v1

This is the official onboarding path for creating an EverArcade world from a fresh clone to a running, packaged, verified world. It intentionally uses high-level platform artifacts so a developer does not need to learn deterministic replay, canonicalization, proof architecture, federation, or certification internals before shipping a first world.

**Onboarding target:** 30 minutes to clone, create a world, run projection, and build a package.

## Quick path

Future CLI:

```bash
everarcade world init --template frontier
cd frontier
everarcade world package
everarcade world verify
everarcade world project
everarcade world deploy
```

Current repository equivalent:

```bash
mkdir -p examples/world-creation-flow/frontier-validation
cp -R examples/reference-certified-world-v1/* examples/world-creation-flow/frontier-validation/
# edit metadata, contract, genesis, continuity, projection, registry, and proof mapping
bash examples/world-creation-flow/frontier-validation/operator/build-world-evr.sh
bash examples/world-creation-flow/frontier-validation/operator/verify.sh
```

## Stage 1: Create World

Future CLI target:

```bash
everarcade world init --template frontier
```

Current equivalent:

1. Copy the reference certified world template into a new world folder.
2. Rename the world-facing metadata to the new world identity.
3. Customize genesis files that define the initial state.
4. Customize the world contract that defines allowed capabilities and lifecycle expectations.

Expected starting files:

| Purpose | Current file |
| --- | --- |
| World manifest | `manifest/world-manifest.toml` |
| Metadata | `metadata/world-metadata.json` |
| Contract | `world-contract/world-contract.toml` |
| Genesis state | `genesis/frontier-state.json` |
| Continuity state | `genesis/continuity-state.json` |
| Continuity policies | `continuity/policies.toml` |
| Projection config | `projection/projection.toml` |
| Registry entry | `registry/frontier-validation-registry-entry.json` |
| Proof mapping | `proofs/proof-mapping.toml` |

## Stage 2: Customize World

Developer-owned customization fields:

| Field | Expected file | Notes |
| --- | --- | --- |
| World name | `metadata/world-metadata.json`, `manifest/world-manifest.toml` | Human-readable identity. |
| World description | `metadata/world-metadata.json` | One-paragraph player/developer summary. |
| World contract | `world-contract/world-contract.toml` | Capabilities, invariants, package rules, certification claims. |
| Genesis state | `genesis/frontier-state.json` | Initial actors, regions, resources, and world clock. |
| Continuity policies | `continuity/policies.toml` | Persistence, restoration, replay, and upgrade policy. |
| Projection layer | `projection/projection.toml` | Read-only state views, replay panels, continuity panels. |

A first-world developer should only edit declarative files. Runtime internals remain hidden behind package, verify, and projection commands.

## Stage 3: Select RustRigs

RustRigs are modular capability packages chosen by the world author. Status terms:

- **required**: must be present for the canonical Frontier flow.
- **optional**: supported by the flow but not required for the first running world.
- **candidate**: planned or scaffold-level capability that can be declared but is not part of the validation gate.
- **certified**: reference package exists and is mapped to certification evidence.

| RustRig | Status | First-world expectation |
| --- | --- | --- |
| identity | required | Every actor, operator, and world package needs stable identity. |
| movement | required | Enables basic avatar/world-position actions. |
| combat | certified | Use the reference combat rig when combat is enabled. |
| inventory | certified | Use the reference inventory rig for item state. |
| resources | optional | Enable when harvesting or resource sinks are part of genesis. |
| crafting | candidate | Declare only if crafting recipes are included. |
| structures | candidate | Declare only if placeable structures are included. |
| market | certified | Use the reference market rig for exchange flows. |
| governance | certified | Use the reference governance rig for admin/world policy updates. |
| quests | optional | Enable when guided objectives exist. |
| factions | optional | Enable when actor groups or reputation exist. |
| continuity | required | Required for persistence and restoration policy. |

## Stage 4: Package World

Future CLI target:

```bash
everarcade world package
```

Current equivalent:

```bash
bash examples/world-creation-flow/frontier-validation/operator/build-world-evr.sh
```

The package step must:

1. Build `dist/world.evr`.
2. Include manifest, metadata, contract, genesis, continuity, projection, registry, proof mapping, and selected RustRig descriptors.
3. Write a deterministic package manifest at `dist/package-manifest.txt`.
4. Fail if required files are missing.

Validate package structure:

```bash
find examples/world-creation-flow/frontier-validation/dist -type f | sort
```

Verify package contents:

```bash
tar -tf examples/world-creation-flow/frontier-validation/dist/world.evr | sort
```

## Stage 5: Verify World

Future CLI target:

```bash
everarcade world verify
```

Current equivalent:

```bash
bash examples/world-creation-flow/frontier-validation/operator/verify.sh
```

Verification covers:

| Check | Current artifact |
| --- | --- |
| Package verification | `dist/world.evr`, `dist/package-manifest.txt` |
| Contract verification | `world-contract/world-contract.toml` |
| Genesis verification | `genesis/frontier-state.json`, `genesis/continuity-state.json` |
| Certification mapping | `proofs/proof-mapping.toml` |

## Stage 6: Run Projection

Future CLI target:

```bash
everarcade world project
```

Current equivalent:

```bash
cat examples/world-creation-flow/frontier-validation/projection/projection.toml
cat examples/world-creation-flow/frontier-validation/genesis/frontier-state.json
cat examples/world-creation-flow/frontier-validation/genesis/continuity-state.json
```

Projection runtime expectations:

- Launch a read-only view of world state.
- Show current region, actor, inventory, resource, and continuity data.
- Show replay status without exposing replay internals.
- Show restoration readiness from continuity state.

## Stage 7: Deploy World

Future CLI target:

```bash
everarcade world deploy
```

Current equivalent:

1. Build and verify `dist/world.evr`.
2. Generate or update `registry/frontier-validation-registry-entry.json`.
3. Hand the package and registry entry to an operator.
4. Include verification evidence from `reports/developer-experience-validation/evidence.md`.
5. Reference the operator handoff in `operator/OPERATOR-HANDOFF.md`.

Deployment does not require the developer to operate federation infrastructure. The developer provides a verified world package plus evidence; the operator performs runtime placement.
