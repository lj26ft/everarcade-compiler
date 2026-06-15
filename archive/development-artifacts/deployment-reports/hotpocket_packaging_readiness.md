# HotPocket Packaging Readiness

| Capability | Status | Notes |
| --- | --- | --- |
| Runtime package | Ready | `everarcade package` invokes deterministic package generation. |
| World package | Ready | World manifests, checkpoint, and replay inputs are packaged. |
| Deployment package | Ready | Deployment manifests, operations, assets, rustrigs, and XRPL anchor records are packaged. |
| Checksum verification | Ready | Aggregate and per-package SHA-256 verification are performed. |
| Contract staging | Ready | `everarcade stage-contract` creates `dist/everarcade-hotpocket-contract/` with packages, config, state, and `start.sh`. |
| Host binary | Partially Ready | Included when `target/release/everarcade-host` exists; not required for scaffold rehearsal. |
| Local rehearsal | Ready | `everarcade rehearse` wraps the existing local contract rehearsal script. |
