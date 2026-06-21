# HotPocket Deployment Compatibility Proof v0.1

## Objective

This proof determines whether EverArcade HotPocket deployment packages match the package and configuration expectations of `hpdevkit`, `evernodecli hp-clean`, `evernodecli hp-deploy`, and a clean three-validator HotPocket development cluster.

The proof is intentionally limited to deployment compatibility. It does not claim gameplay correctness, multiplayer correctness, replay correctness, federation, Evernode lease operation, XRPL settlement, Xahau settlement, or production readiness.

## Proof Layout

The canonical proof assets live under `runtime/hotpocket-deployment-proof/`:

- `templates/variant-a/` contains a direct Node.js contract package with `package.json`, `contract.js`, and `patch.cfg`.
- `templates/variant-b/` contains a Node.js contract package with an executable `contract` wrapper, `contract.js`, and `patch.cfg`.
- `templates/variant-c/` contains the Creator SDK generated package shape.
- `templates/variant-d/` contains the HotPocket adapter generated package shape.
- `validation/hotpocket-deployment-proof.js` performs package inspection, live deployment, executable resolution, dependency, launch, proposal, discovery, and certification checks.
- `reports/` receives proof-local report output. The same reports are mirrored to repository-level `reports/`.

## Canonical Package Matrix

| Variant | Package shape | `bin_path` | `bin_args` | Purpose |
| --- | --- | --- | --- | --- |
| A | Node contract with `package.json`, `contract.js`, and `patch.cfg` | `node` | `contract.js` | Proves the minimum direct Node contract package shape. |
| B | Node contract with executable wrapper and `patch.cfg` | `./contract` | `--hotpocket` | Proves wrapper-based executable discovery. |
| C | Creator SDK generated shape | `node` | `contract.js --creator-sdk` | Proves Creator SDK output can carry explicit deployment metadata. |
| D | HotPocket adapter generated shape | `node` | `contract.js --adapter` | Proves adapter output can carry explicit deployment metadata. |

Every variant must be independently deployable by running `evernodecli hp-clean` followed by `evernodecli hp-deploy <variant-path>` with no manual edits.

## Package Requirements

A compatible package must include:

1. `package.json` with name, version, `main`, and a start script.
2. `contract.js` as the Node contract entrypoint unless the package variant deliberately adds an executable wrapper.
3. `patch.cfg` with non-placeholder `bin_path` and non-empty `bin_args`.
4. A resolvable executable target:
   - `node` plus `contract.js`, or
   - an executable wrapper present inside the package.
5. Dependency metadata in `package.json` and packaged dependency payload when dependencies are declared.

The proof specifically rejects generated configurations that leave `bin_path = "<your contract binary here>"` or an empty `bin_args` value.

## Deployment Lifecycle

The live deployment proof uses only standard deployment commands:

```bash
evernodecli hp-clean
evernodecli hp-deploy runtime/hotpocket-deployment-proof/templates/<variant>
```

The scripts forbid manual container modification, `docker exec` patching, generated `hp.cfg` edits, generated `patch.cfg` edits, and runtime binary patching. A PASS requires `hp-clean` and `hp-deploy` to succeed for every variant.

## Executable Discovery

After deployment, the executable-resolution proof inspects generated `hp.cfg` files under the configured cluster root. Set one of the following environment variables to the root of the HotPocket development cluster filesystem before running executable, proposal, validation, or certification checks:

- `HOTPOCKET_CLUSTER_ROOT`
- `HPDEVKIT_CLUSTER_ROOT`
- `EVERARCADE_HOTPOCKET_CLUSTER_ROOT`

The proof requires at least three generated `hp.cfg` files, non-placeholder `bin_path`, non-empty `bin_args`, and a launchable executable target on all validator nodes.

## Dependency Handling

The dependency proof launches each package using its declared deployment command and verifies that no `Cannot find module` failure occurs. Packages with no declared dependencies do not require `node_modules`. Packages that declare dependencies must package `node_modules` or another deployment-visible dependency payload before certification can pass.

## Validator Startup and Proposal Participation

The cluster proposal proof validates the deployment side of validator participation. A PASS requires:

- `HOTPOCKET_VALIDATOR_COUNT >= 3`
- `HOTPOCKET_PROPOSAL_COUNT >= 3`
- no `votes:1 needed:3`
- no `votes:2 needed:3`
- no `Not enough peers proposing`

The validator and proposal counts are captured from the live validation environment because HotPocket log formats vary across hpdevkit releases. The proof also scans available HotPocket logs below the configured cluster root for known proposal-failure diagnostics.

## Reports

The validation scripts generate the required reports:

| Script | Report |
| --- | --- |
| `scripts/run_hotpocket_package_inspection.sh` | `reports/hotpocket_package_inspection_report.json` |
| `scripts/run_hotpocket_deployment_proof.sh` | `reports/hotpocket_deployment_compatibility_report.txt` |
| `scripts/run_hotpocket_executable_resolution.sh` | `reports/hotpocket_executable_resolution_report.txt` |
| `scripts/run_hotpocket_dependency_proof.sh` | `reports/hotpocket_dependency_packaging_report.txt` |
| `scripts/run_hotpocket_launch_proof.sh` | `reports/hotpocket_contract_launch_report.txt` |
| `scripts/run_hotpocket_cluster_proposal_proof.sh` | `reports/hotpocket_cluster_proposal_report.txt` |
| `scripts/validate_hotpocket_deployment.sh` | all reports plus `reports/hotpocket_deployment_discovery_report.json` |
| `scripts/certify_hotpocket_deployment.sh` | all reports plus `reports/hotpocket_deployment_certification_report.txt` |

## Compatibility Findings

The proof converts the current observed failure into an explicit gate:

```text
bin_path = "<your contract binary here>"
bin_args = ""
```

A deployment that produces the placeholder `bin_path`, an empty `bin_args`, missing executables, dependency resolution failures, contract launch failures, or insufficient validator proposals is classified as not proven. The final certification line is emitted only by `scripts/certify_hotpocket_deployment.sh` and is PASS only when every required gate passes:

```text
HotPocket Deployment Compatibility Proof v0.1: PASS
```

## Deployment Limitations and Non-Claims

This milestone does not prove:

- contract execution correctness;
- gameplay;
- multiplayer;
- replay;
- federation;
- civilization hosting;
- Evernode lease hosting;
- XRPL settlement;
- Xahau settlement;
- production operation.

It proves only HotPocket deployment compatibility for packages that pass the package, deployment, executable, dependency, launch, and proposal gates without manual patching.
