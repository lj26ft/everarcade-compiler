# dry-run-vs-live

`./scripts/deploy_proof.sh` defaults to dry-run adapters so deployment verification is merge-safe and deterministic.

## Defaults (safe)

- `DRY_RUN=1`
- `XRPL_ADAPTER=dry-run`
- `IPFS_ADAPTER=dry-run`
- `EVERNODE_ADAPTER=dry-run`
- live integration scripts are skipped unless explicitly enabled.

## Live feature flags

Only set these in a real operator/testnet environment:

- `ENABLE_XRPL_LIVE=1` to run `./scripts/local_xrpl_testnet.sh`
- `ENABLE_IPFS_LIVE=1` to run `./scripts/local_ipfs_publish.sh`
- `ENABLE_EVERNODE_LIVE=1` to enable Evernode live adapter hooks

## Example

Dry-run proof (recommended):

```bash
./scripts/deploy_proof.sh
```

Dry-run proof with explicit live XRPL/IPFS checks:

```bash
ENABLE_XRPL_LIVE=1 ENABLE_IPFS_LIVE=1 ./scripts/deploy_proof.sh
```
