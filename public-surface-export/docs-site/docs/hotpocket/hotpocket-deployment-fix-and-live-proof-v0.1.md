# HotPocket Deployment Fix & Live Contract Deployment Proof v0.1

## Objective

This milestone proves the deployment path between EverArcade HotPocket deployment packages and `hpdevkit` without manual container modification. A PASS means a clean package can be deployed, launched by HotPocket, observed on a three-validator development cluster, and exercised by a client round trip.

The milestone intentionally stops at HotPocket live deployment proof. It does **not** prove Evernode leases, production hosting, federation, gameplay, multiplayer, XRPL settlement, replay, or civilization runtime behavior.

## Root Cause Analysis

Live investigation showed the EverArcade runtime was not the blocker. The blocker was the proof and deployment compatibility layer around HotPocket metadata:

1. `patch.cfg` parsing only understood `key=value`, while current deployment templates may emit JSON.
2. Fresh `hp-deploy` output can retain placeholder executable metadata such as `<your contract binary here>` or empty `bin_args`.
3. The proof harness depended on manually configured cluster roots, so it could report zero `hp.cfg` files even when `hpdevkit` containers were running.
4. Validator and client proof reports were not tied to generated deployment configuration, live logs, and client-visible output in one certification path.

## Parser Defect and Canonical Parsing

`runtime/hotpocket-deployment-proof/validation/hotpocket-deployment-proof.js` now parses deployment configuration as JSON first and falls back to HotPocket `key=value` syntax. This accepts both canonical forms:

```json
{
  "bin_path": "node",
  "bin_args": "contract.js"
}
```

```text
bin_path=node
bin_args=contract.js
```

The proof reports `bin_path_present` and `bin_args_present` before deployment and refuses to continue cleanly when either value is missing or when `bin_path` contains the known placeholder.

## Deployment Metadata Requirements

Before `hp-deploy`, every package must provide:

- `patch.cfg`;
- populated `bin_path`;
- populated `bin_args`;
- no `<your contract binary here>` placeholder.

After `hp-deploy`, the proof validates generated artifacts and writes:

```text
reports/hotpocket_generated_config_report.txt
```

The post-deploy report requires generated `hp.cfg`, generated `patch.cfg`, and generated contract/ledger state evidence.

## hpdevkit Discovery

The proof no longer requires manual cluster-root environment variables. It still honors `HOTPOCKET_CLUSTER_ROOT`, `HPDEVKIT_CLUSTER_ROOT`, and `EVERARCADE_HOTPOCKET_CLUSTER_ROOT`, but it also automatically inspects Docker:

- `docker ps` for HotPocket/hpdevkit containers;
- `docker inspect` for mounted node roots;
- `docker volume ls` and `docker volume inspect` for hpdevkit volumes such as `hpdevkit_default_node_*`.

Discovery output is mirrored to:

```text
reports/hotpocket_cluster_discovery_report.json
```

## Executable Resolution and hp.cfg Validation

Every discovered `hp.cfg` is parsed with the same JSON-first/fallback parser. The proof validates:

- `bin_path` is populated;
- `bin_args` is populated;
- the placeholder is absent;
- the executable target can be resolved and launched.

The hp.cfg-specific output is:

```text
reports/hotpocket_hp_cfg_validation_report.txt
```

The executable-resolution output remains:

```text
reports/hotpocket_executable_resolution_report.txt
```

## Contract Launch Verification

The local package launch proof still verifies package launchability before deployment. The live launch proof additionally scans generated cluster configuration and live node logs, rejecting known fatal launch failures:

- `Contract process execve() failed`;
- `Cannot find module`;
- `<your contract binary here>`.

Live launch output is:

```text
reports/hotpocket_live_contract_launch_report.txt
```

## Validator Participation

Validator participation is proven from discovered node roots, generated `hp.cfg` files, optional environment counters, and live HotPocket logs. A PASS requires three validators online and at least three participation/proposal observations, while rejecting:

- `votes:1 needed:3`;
- `votes:2 needed:3`;
- `Not enough peers proposing`.

Validator proof output is:

```text
reports/hotpocket_validator_participation_report.txt
```

## Client Round-Trip Execution

The deployment proof can discover websocket endpoints from `HOTPOCKET_SERVERS`, `HP_SERVERS`, or Docker-published HotPocket ports. It then runs the existing HotPocket client proof against the deployed contract and submits:

```json
{
  "action": "ping"
}
```

A PASS requires:

- client connected;
- input accepted;
- contract executed;
- output returned.

Client proof output is:

```text
reports/hotpocket_client_roundtrip_report.txt
```

## Certification

Run:

```bash
bash scripts/certify_hotpocket_deployment.sh
```

The final line is PASS only when all deployment gates pass:

```text
HotPocket Deployment Fix & Live Contract Deployment Proof v0.1: PASS
```

## Lessons Learned

- Deployment metadata must be treated as a compatibility contract, not as incidental template text.
- `patch.cfg` format drift must be tolerated by canonical parsing, not by manual container edits.
- Proof harnesses must discover the live hpdevkit topology automatically, because Docker volume and container names differ across environments.
- Generated `hp.cfg` is the authoritative deployment result and must be validated separately from the source package.
- Validator participation and client-visible output are separate gates; both are required before claiming live HotPocket deployment proof.

## Explicit Non-Claims

This milestone does **not** prove:

- Evernode deployment;
- production hosting;
- federation;
- gameplay;
- multiplayer;
- XRPL settlement;
- civilization runtime.

It proves only:

```text
A clean HotPocket deployment package can be deployed,
executed,
participate in consensus,
and return output to a client.
```

## Successor Milestone

Upon PASS, the successor milestone is:

```text
HotPocket Consensus Gameplay Proof v0.1
```

The next goal is to execute real EverArcade gameplay actions through HotPocket consensus rather than local runtime simulation.
