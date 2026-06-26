# EverArcade Architecture

EverArcade is a deterministic world-runtime and creator-tooling ecosystem. Its first open-source milestone is focused on local proof: create a world, package it reproducibly, execute it, and verify the evidence it emits.

## System flow

```text
Developer
  ↓
World Factory
  ↓
world.evr
  ↓
Verification
  ↓
Registry
  ↓
Vault
  ↓
Operator
  ↓
Player
```

## Layers

### Developer

Developers use the Creator SDK, templates, examples, RustRigs, and documentation to author local worlds. The developer-facing promise today is a reproducible first-world path, not a production marketplace or public testnet.

### World Factory

The World Factory turns source material, manifests, fixtures, and packaging rules into a canonical world artifact. It is the boundary between editable source and generated release evidence.

### `world.evr`

`world.evr` is the packaged world artifact produced by the World Factory. It is generated output, reviewed through gates, and distributed through releases or release bundles rather than treated as ordinary source.

### Verification

Verification consumes world packages, replay material, attestations, receipts, journals, checkpoints, and manifests. Its purpose is to prove that a world package and its evidence match the documented deterministic expectations under pinned local conditions.

### Registry

The registry layer represents discoverability and metadata: what worlds, packages, templates, runtime manifests, and evidence records exist. In this repository, registry domains are largely reference or scaffold unless `MATURITY.md` states otherwise.

### Vault

The vault layer represents preserved evidence and reproducibility inputs: package records, trusted fixture keys, offline vendor material, review bundles, proofs, and historical milestones. The project treats these as engineering record, not disposable clutter.

### Operator

Operators run local runtime processes, deployment rehearsals, Evernode experiments, node lifecycle scripts, and diagnostics. Operator paths are experimental or scaffold-level unless explicitly classified otherwise.

### Player

Players are the eventual consumers of running worlds through clients, gateways, or portals. In this repository the player-facing surface is not the canonical proof path; first-time contributors should start with local CLI verification instead.

## Trust and maturity boundaries

- `TRUST_ROOT.md` stays visible because it is part of the public trust model.
- RC1, RC2, and review bundles remain visible launch-history evidence.
- Renderer, history, federation, public testnet, settlement, GPU marketplace, and commercial domains should be treated as scaffold or experimental unless `MATURITY.md` says otherwise.
- PASS output from a local script proves only the named local proof under documented conditions.

## Where to go next

- Start: `README.md`
- Contributor workflow: `CONTRIBUTING.md`
- Repository map: `REPOSITORY_MAP.md`
- Maturity reality check: `MATURITY.md`
- Scripts map: `scripts/MANIFEST.md`
- Source/artifact policy: `REPOSITORY_POLICY.md`
