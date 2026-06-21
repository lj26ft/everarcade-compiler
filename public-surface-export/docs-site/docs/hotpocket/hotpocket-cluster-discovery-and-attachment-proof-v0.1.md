# HotPocket Cluster Discovery & Attachment Proof v0.1

## Purpose

This milestone proves only that EverArcade validation tooling can automatically discover, attach to, and inspect a live hpdevkit HotPocket cluster. It intentionally does not claim gameplay, receipt, journal, checkpoint, federation, XRPL, Evernode, multiplayer, or civilization-runtime correctness.

Final certification is emitted to:

```text
reports/hotpocket_cluster_discovery_and_attachment_certification_report.txt
```

A passing run ends with:

```text
HotPocket Cluster Discovery & Attachment Proof v0.1: PASS
```

## Discovery Architecture

The proof is implemented by `runtime/hotpocket-deployment-proof/validation/hotpocket-deployment-proof.js` and is exposed through:

```bash
bash scripts/run_hotpocket_cluster_discovery_proof.sh
```

or:

```bash
cd runtime/hotpocket-deployment-proof
npm run discovery
```

The harness is deliberately read-only with respect to the live HotPocket cluster. It uses allowed inspection surfaces only:

- `docker ps`
- `docker ps -a`
- `docker inspect`
- `docker volume ls`
- `docker volume inspect`
- `docker network ls`
- host filesystem inspection of discovered mountpoints
- TCP and WebSocket reachability checks against discovered published ports

It does not use `docker exec`, manual path edits, environment-variable patching, or container mutation.

## Docker Discovery Flow

The Docker phase captures container, volume, and network state and writes:

```text
reports/hotpocket_docker_discovery_report.json
```

The report includes:

- raw `docker ps` rows
- raw `docker ps -a` rows
- raw `docker volume ls` rows
- raw `docker network ls` rows
- HotPocket/hpdevkit container IDs
- container names
- status/state
- image names
- published ports
- mount metadata
- labels and network settings needed by later phases

The expected hpdevkit cluster shape is:

```text
hpdevkit_default_node_1
hpdevkit_default_node_2
hpdevkit_default_node_3
hpdevkit_default_deploymgr
```

The Docker proof passes when those expected active containers are found, or when an equivalent active cluster has at least three node containers and an active deploy manager.

## Volume Discovery Flow

The volume phase correlates container mounts with Docker named volumes and writes:

```text
reports/hotpocket_volume_discovery_report.json
```

The harness searches for `/hpdevkit_vol` through:

1. `docker inspect` container mount records.
2. `docker volume ls` candidate names.
3. `docker volume inspect` mountpoints.
4. Host filesystem existence checks for resolved mountpoints.

Each discovered mount records:

- container name and ID
- host path
- container path
- Docker volume name
- mount type

## Node Root Discovery Flow

The node-root phase writes:

```text
reports/hotpocket_node_root_discovery_report.json
```

Starting from discovered volume host paths, the harness performs bounded filesystem discovery for node roots that expose all required directories:

```text
cfg/
contract_fs/
ledger_fs/
log/
```

The expected canonical nodes are:

```text
node1
node2
node3
```

A node root is accepted only when all required directories exist.

## hp.cfg Discovery Flow

The hp.cfg phase writes:

```text
reports/hotpocket_hp_cfg_discovery_report.json
```

For each validator node root, the harness inspects:

```text
cfg/hp.cfg
```

The parser accepts JSON-style config files and HotPocket-style `key=value` config files. It captures:

- public key metadata when present
- contract ID metadata when present
- consensus-related configuration keys
- `bin_path`
- `bin_args`

The phase rejects missing `hp.cfg` files and placeholder executable metadata such as `<your contract binary here>`.

## Contract Filesystem Discovery Flow

The contract phase writes:

```text
reports/hotpocket_contract_discovery_report.json
```

For each node, the harness inspects `contract_fs/` and discovers:

- `contract.js`
- `package.json`
- `patch.cfg`
- `node_modules/` when present

It records package name, dependencies, and launch metadata from `patch.cfg`.

## Ledger Discovery Flow

The ledger phase writes:

```text
reports/hotpocket_ledger_discovery_report.json
```

For each `ledger_fs/` root, the harness captures:

- ledger root path
- shard directories
- SQLite-like files (`.db`, `.sqlite`, `.sqlite3`)
- continuity and validator-list files such as continuity files and `unl.json`

This phase proves ledger-root discovery only. It does not prove ledger advancement or consensus success.

## Log Discovery Flow

The log phase writes:

```text
reports/hotpocket_log_discovery_report.txt
```

For each node `log/` directory, the harness locates:

```text
hp.log
contract.log
```

It summarizes whether log text contains evidence of:

- validator startup
- contract startup
- proposal/round/consensus activity
- error-like terms

A PASS requires validator log files to be discoverable. Proposal activity is reported as observed or not observed because this milestone does not claim consensus participation.

## Endpoint Discovery Flow

The endpoint phase writes:

```text
reports/hotpocket_endpoint_discovery_report.json
```

Published host ports are collected from Docker inspect network settings. The expected hpdevkit ports are commonly:

```text
8081
8082
8083
```

However, the harness does not hard-code those ports. It uses whatever host ports Docker actually published for the discovered containers.

Each endpoint records:

- container name
- container port
- host
- host port
- protocol (`ws`)
- TCP reachability
- WebSocket handshake reachability

## Attachment Map Generation

The attachment phase writes:

```text
reports/hotpocket_cluster_attachment_report.json
```

The harness joins node-root, container, config, log, contract, ledger, and endpoint discoveries into an attachment map shaped as:

```json
{
  "node1": {
    "container": "hpdevkit_default_node_1",
    "container_id": "...",
    "hp_cfg": ".../node1/cfg/hp.cfg",
    "log": ".../node1/log/hp.log",
    "contract_root": ".../node1/contract_fs",
    "ledger_root": ".../node1/ledger_fs",
    "endpoint": "ws://127.0.0.1:8081"
  }
}
```

The attachment phase passes only when every discovered validator node can be attached to a container, config file, log file, contract root, ledger root, and endpoint without manual intervention.

## Validation Methodology

The certification command runs discovery twice in a single process. It then validates that both runs produce the same stable signature for:

- active HotPocket containers
- node roots
- `hp.cfg` files
- contract roots
- endpoints

The consistency report is written to:

```text
reports/hotpocket_discovery_consistency_report.txt
```

Certification passes only when all of the following are true:

- containers discovered
- volumes discovered
- node roots discovered
- hp.cfg files discovered
- contract roots discovered
- ledger roots discovered
- logs discovered
- endpoints discovered
- attachment map generated
- discovery reproducible

## Failure Modes

Common failure modes and likely causes:

- **Docker unavailable**: Docker CLI is missing or the daemon is not reachable.
- **Containers missing**: hpdevkit cluster is not running, or container names/images do not include detectable HotPocket/hpdevkit markers.
- **Volume missing**: containers do not mount `/hpdevkit_vol`, or the Docker volume mountpoint is not visible to the host user.
- **Node roots missing**: volume contents do not expose `cfg/`, `contract_fs/`, `ledger_fs/`, and `log/` under validator node roots.
- **hp.cfg missing or placeholder**: deployment generated incomplete validator config, or executable metadata still contains placeholder values.
- **Contract files missing**: deployed contract filesystem does not include `contract.js`, `package.json`, or `patch.cfg`.
- **Logs missing**: validators have not started far enough to create `hp.log` or `contract.log`, or logs are written under an unexpected path.
- **Endpoints unreachable**: Docker did not publish websocket ports, ports are bound to a different host interface, or the validator websocket server is not accepting handshakes.
- **Inconsistent discovery**: cluster is starting/stopping during validation or Docker metadata changes between the two discovery passes.

## Lessons Learned

The deployment compatibility proof showed that successful deployment and manual inspection are insufficient. A validation harness must be able to construct a complete, deterministic attachment model from observable runtime state. This proof turns discovery into a first-class validation target by saving every intermediate artifact and requiring a reproducible attachment map before successor consensus-participation checks can run.

## Successor Milestone

After this proof passes, the successor milestone is:

```text
HotPocket Consensus Participation Proof v0.1
```

That successor should use the attachment map from this milestone to prove that all discovered validators actively participate in HotPocket consensus and that client submissions advance through rounds.
