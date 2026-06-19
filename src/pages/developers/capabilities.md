# Developer Capability Matrix

EverArcade is explicit about maturity. This website matrix mirrors the active repository classifications in `MATURITY.md` and the implementation-focused runtime status in `docs/runtime-capabilities.md`. It should be read as a readiness guide, not as a production support promise.

No open-source subsystem is currently classified as **Production**.

| Capability | Website maturity | Repo evidence today | Current boundary / next step |
| --- | --- | --- | --- |
| Execution Core | **Alpha** | Local deterministic execution and replay-oriented proof paths exist. | Needs broader adversarial hardening and release certification before production claims. |
| Runtime | **Alpha** | Local package execution, receipts, journals, checkpoints, and replay verification are the primary proven path. | Production guarantees, operator drills, and certification gates remain incomplete. |
| Creator SDK | **Alpha** | Local project creation, build, manifest validation, packaging, and play-local flows are supported. | Onboarding workflows are usable locally while distribution policy keeps maturing. |
| World Packages | **Alpha** | Canonical package shape is documented and used by local flows. | Marketplace distribution and upgrade policies remain evolving. |
| RustRigs | **Alpha** | Reusable gameplay domain libraries exist for authoring and examples. | Not yet a production standard library or mature module ecosystem. |
| World Contracts | **Experimental** | ABI and contract boundaries are documented and partially proven. | Public API stability is not final; richer authoring patterns are still evolving. |
| Evernode Deployment | **Experimental** | Deployment and lease proof material exists. | Public operator guarantees and commercial deployment automation are not established. |
| Backup / Restore | **Partial runtime capability** | Checkpoint-backed backup manifests and restore foundations exist. | Production backup policy, restore certification, and operator drills remain gated. |
| Runtime Health and Metrics | **Partial runtime capability** | Health and metrics primitives exist. | Production observability and operational SLOs remain incomplete. |
| Federation | **Scaffold** | Design notes, prototypes, recovery modules, and tests exist. | Treat federation as a scaffold-level runtime domain, not real multiplayer production federation. |
| History / Observer Runtime | **Scaffold** | Historical replay and observer paths are projection-oriented runtime domains. | Treat history/observer capability as scaffold-level until production gates exist. |
| Renderer / Projection | **Scaffold** | Renderer and projection areas are documented and modeled. | Not the canonical proof path and not a production client runtime. |
| GPU Marketplace / GPU Runtime | **Scaffold** | GPU marketplace, worker, provider, and deterministic capability-profile material exists. | Future hosting economics and verification only; no production GPU marketplace or acceleration guarantee. |
| XRPL Settlement | **Scaffold** | Settlement boundaries and proof documents exist. | Live settlement is not a production system. |
| Xahau Hooks | **Scaffold** | Hook directories document intended boundaries and proof surfaces. | Not production-ready. |
| Developer Portal | **Scaffold** | Portal directories and docs exist. | Contributors should rely on CLI and docs first. |
| Player Gateway | **Scaffold** | Player-facing concepts and guide material exist. | Production player experience is not complete. |
| Commercial Revenue / Marketplace | **Scaffold** | Revenue, discovery, and marketplace plans are reference material. | Not an active production marketplace or revenue system. |
| Public Testnet | **Planned** | Public-testnet records and scripts are model artifacts only. | Public-testnet operation is not currently supported as production or alpha capability. |
| ZK Integration | **Planned** | ZK proof integration appears only as a planned runtime capability. | No production runtime implementation exists. |

## Runtime capability snapshot

The current implementation-focused runtime matrix classifies deterministic WASM execution, package loading, state roots, receipt generation, journal persistence, checkpoint creation, replay verification, and runtime recovery as **Implemented** foundations. Backup, restore, runtime upgrade, health/metrics, world runtime, federation recovery, distributed receipt propagation, checkpoint synchronization, and Evernode deployment automation are **Partial**. Multi-host federation, renderer streaming, historical replay/observer runtime, XRPL integration, and creator marketplace are **Scaffold**. ZK integration is **Planned**.

## How to read maturity

- **Alpha** means usable for local or limited workflows with known gaps.
- **Experimental** means prototypes demonstrate direction but may change substantially.
- **Partial runtime capability** means foundations exist in runtime docs/code, but production automation, policy, or certification gates are incomplete.
- **Scaffold** means a directory, interface, documentation page, model, or placeholder shape exists; do not treat it as a working product.
- **Planned** means the direction is on the roadmap but lacks meaningful implementation in this repository.

## Related technical docs

- [System overview](/docs/architecture/system-overview)
- [Runtime capability matrix](/docs/runtime-capabilities)
- [WASM runtime](/docs/architecture/runtime/wasm-runtime)
- [Replay engine](/docs/architecture/runtime/replay-engine)
- [Checkpoint system](/docs/architecture/runtime/checkpoint-system)
- [Federation](/docs/architecture/federation)
- [World packaging](/docs/architecture/world-packaging)
- [GPU hosting strategy](/docs/architecture/gpu_hosting_strategy)
