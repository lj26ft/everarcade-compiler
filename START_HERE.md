# Start Here

Welcome to EverArcade. This is the orientation page for first-time visitors who want to understand `world.evr`, install the project, verify it locally, and find the canonical documentation without hunting through the repository.

## What is world.evr?

`world.evr` is EverArcade's portable world package format: a deterministic bundle of world metadata, runtime inputs, receipts, and replay-verification material that can be inspected and reproduced locally.

## Why it exists

EverArcade exists to make digital worlds easier to preserve, replay, and independently verify. Instead of treating a running game session as an opaque event, EverArcade focuses on deterministic execution evidence: packages, receipts, journals, checkpoints, transcripts, and replay proof material.

The current repository is an open-source candidate focused on local developer onboarding and proof paths. Treat renderer, history, federation, settlement, marketplace, and production network areas according to their maturity labels in [`MATURITY.md`](MATURITY.md).

## Architecture overview

At a high level:

1. **Creator tooling** builds or initializes a local world project.
2. **World packages** describe deterministic content and runtime boundaries.
3. **Runtime execution** runs the world and emits evidence.
4. **Verification tooling** checks package integrity and replay evidence.
5. **Documentation and examples** explain what is stable today versus scaffold or future-facing.

Read the full architecture next in [`ARCHITECTURE.md`](ARCHITECTURE.md).

## Five-minute quick start

For the shortest clone/install/run/verify path, use [`QUICKSTART.md`](QUICKSTART.md).

```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

Expected result: prerequisites pass, onboarding passes, and the reference certified world verifies successfully.

## Documentation links

- Documentation site: <https://docs.everarcade.games>
- Website: <https://everarcade.games>
- Documentation directory: [`DOCS.md`](DOCS.md)
- Compiler documentation root: [`docs/index.md`](docs/index.md)
- Documentation policy: [`docs/DOCUMENTATION_POLICY.md`](docs/DOCUMENTATION_POLICY.md)
- Repository map: [`REPOSITORY_MAP.md`](REPOSITORY_MAP.md)

## Verification links

- Independent verification overview: [`VERIFICATION.md`](VERIFICATION.md)
- Reference certified world verifier: [`examples/reference-certified-world-v1/operator/verify.sh`](examples/reference-certified-world-v1/operator/verify.sh)
- Developer onboarding validator: [`scripts/validate_developer_onboarding.sh`](scripts/validate_developer_onboarding.sh)
- Prerequisite checker: [`scripts/check_prerequisites.sh`](scripts/check_prerequisites.sh)

## Release links

- Latest GitHub releases: <https://github.com/lj26ft/everarcade-compiler/releases>
- RC2 reviewer path: [`README.md#rc2-independent-reviewer-path`](README.md#rc2-independent-reviewer-path)
- Release/readiness docs: [`docs/open-source/v0.1-public-release-readiness.md`](docs/open-source/v0.1-public-release-readiness.md)

## Suggested reading order

1. [`README.md`](README.md)
2. [`START_HERE.md`](START_HERE.md)
3. Documentation site: <https://docs.everarcade.games>
4. [`QUICKSTART.md`](QUICKSTART.md)
5. [`ARCHITECTURE.md`](ARCHITECTURE.md)
6. Specifications under [`docs/`](docs/)
7. [`VERIFICATION.md`](VERIFICATION.md)
8. [`examples/`](examples/)
