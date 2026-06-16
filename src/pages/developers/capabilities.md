# Developer Capability Matrix

EverArcade is explicit about maturity. The website describes what the platform is for; the documentation portal remains the implementation layer for commands, formats, and runtime details.

| Capability | Current | In Progress | Planned |
| --- | --- | --- | --- |
| World Contracts | Contract API, ABI boundary, and examples exist for defining world rules. | Better authoring ergonomics and reference patterns. | Richer reusable contract libraries. |
| Deterministic Runtime | WASM runtime and deterministic execution architecture are documented. | More profiling, benchmarking, and isolation hardening. | Broader production certification workflows. |
| Replay Verification | Replay engine and verification concepts are documented. | More accessible receipts and verifier flows. | Public verifier tooling for operators and communities. |
| Checkpoint Restore | Checkpointing, restoration, and recovery paths are documented. | More operator recovery automation. | Community-facing continuity recovery procedures. |
| Federation | Federation architecture and runtime docs exist. | Practical operator coordination patterns. | Larger multi-operator continuity networks. |
| World Packaging | Canonical package and game package formats are documented. | Clearer template packaging defaults. | Marketplace-ready package validation. |
| RustRigs | Reusable gameplay module concept and scaffolds exist. | Expanded examples for common world systems. | Mature module ecosystem for creators. |
| Marketplace | Discovery and creator marketplace records/scaffolds exist. | Packaging and distribution workflows. | Public marketplace for worlds, modules, and services. |
| GPU Runtime | GPU hosting and projection models are documented as runtime domains. | Scaffold-level validation and clearer boundaries. | Production GPU runtime acceleration where appropriate. |
| Governance | Governance concepts appear in world architecture and roadmap materials. | Practical standards for upgrades and stewardship. | Community governance tooling and review processes. |
| Settlement | Settlement intent and ledger boundary docs exist. | Clearer separation between gameplay, ownership, and settlement. | Production settlement integrations after architecture review. |

## How to read maturity

- **Current** means a documented architecture, scaffold, API, template, or working implementation exists in the repository.
- **In Progress** means the project is actively turning capability into repeatable developer/operator workflows.
- **Planned** means the direction is part of the public roadmap, but should not be treated as production-ready.

## Related technical docs

- [System overview](/docs/architecture/system-overview)
- [WASM runtime](/docs/architecture/runtime/wasm-runtime)
- [Replay engine](/docs/architecture/runtime/replay-engine)
- [Checkpoint system](/docs/architecture/runtime/checkpoint-system)
- [Federation](/docs/architecture/federation)
- [World packaging](/docs/architecture/world-packaging)
- [GPU hosting strategy](/docs/architecture/gpu_hosting_strategy)
