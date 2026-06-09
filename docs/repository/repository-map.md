# EverArcade Repository Map

This map is the conceptual ownership and maturity guide for new contributors. It classifies subsystems by what a maintainer should expect today, not by long-term ambition.

## Maturity categories

- **Production Candidate**: usable as a maintained v0.1 component with validation coverage, but still pre-production unless explicitly released.
- **Functional Prototype**: runnable or useful locally, but not hardened for production or public operation.
- **Proof Scaffold**: exists to model, validate, or record a claim; do not treat as production implementation.
- **Documentation Only**: explanatory material or planning surface.
- **Experimental**: exploratory code or domain model with unstable support expectations.

## Top-level ownership map

| Path | Conceptual owner | Why it exists | Current maturity |
| --- | --- | --- | --- |
| `.cargo/` | Build/release maintainers | Forces the intended Cargo source policy for this clone. | Proof Scaffold |
| `arena-vanguard*/` | Template/gameplay maintainers | Holds Arena Vanguard sample assets, manifests, package records, replay/checkpoint examples, and browser/gateway prototypes. | Functional Prototype / Proof Scaffold |
| `benchmarks/` | Runtime performance maintainers | Describes benchmark expectations and performance experiments. | Documentation Only |
| `certification/` | Release/proof maintainers | Stores certification-oriented records and supporting proof material. | Proof Scaffold |
| `clients/` | Client integration maintainers | Client-facing prototypes and integration surfaces. | Experimental |
| `commercial-revenue/` | Business-model maintainers | Models commercial revenue concepts only. | Proof Scaffold |
| `compiler/` | Compiler/toolchain maintainers | Compiler-facing experiments and source surfaces. | Experimental |
| `contract-api/` | Runtime contract maintainers | Rust crate for contract-facing API boundaries. | Functional Prototype |
| `contracts/` | Guest contract maintainers | Sample/proof guest contracts, including WASM proof inputs. | Functional Prototype |
| `control-plane/` | Operations maintainers | Control-plane crate/prototype boundaries. | Proof Scaffold |
| `creator-examples/` | Creator SDK maintainers | Example creator projects and fixtures. | Functional Prototype |
| `creator-marketplace/` | Marketplace maintainers | Marketplace modeling and records. | Proof Scaffold |
| `creator-sdk/` | Creator experience maintainers | Node CLI, templates, and creator workflow prototype. | Functional Prototype |
| `deploy/`, `deployment/` | Operator/deployment maintainers | Deployment scripts, records, and runtime operation models. | Proof Scaffold |
| `developer-portal/` | Developer experience maintainers | Portal model and contributor-facing concepts. | Proof Scaffold |
| `docs/` | Documentation maintainers | Canonical architecture, onboarding, runtime, repository, and policy documentation. | Documentation Only |
| `everarcade-abi/` | Runtime contract maintainers | Shared ABI crate for host/guest boundaries. | Functional Prototype |
| `everarcade-host/` | Host runtime maintainers | Host crate for runtime integration work. | Functional Prototype |
| `evernode/`, `provider-evernode/`, `hotpocket/` | Operator/runtime integration maintainers | Evernode/HotPocket provider models and integration proof material. | Proof Scaffold |
| `examples/` | Developer experience maintainers | General examples and fixtures. | Functional Prototype |
| `execution-core/` | Determinism/runtime maintainers | Core deterministic execution crate. | Functional Prototype |
| `federation/` | Federation maintainers | Federation models, simulations, and validation scaffolds. | Proof Scaffold |
| `frontend/`, `frontend-gateway/`, `gateway/`, `player-gateway/` | Frontend/gateway maintainers | Gateway and player-facing prototypes. | Experimental / Proof Scaffold |
| `game-discovery/`, `marketplace/` | Discovery/marketplace maintainers | Discovery and marketplace models. | Proof Scaffold |
| `gpu/` | GPU marketplace maintainers | GPU provider/marketplace modeling only. | Proof Scaffold |
| `handoff/`, `release/` | Release maintainers | Release/handoff records and packaging support material. | Proof Scaffold |
| `hooks/` | Runtime integration maintainers | Hook examples and integration fixtures. | Experimental |
| `metrics/` | Observability maintainers | Health/readiness metric models across domains. | Proof Scaffold |
| `node/`, `routing/` | Protocol node maintainers | Node and routing model surfaces. | Proof Scaffold |
| `player_sessions/` | Runtime evidence maintainers | Local/generated session evidence fixtures. | Proof Scaffold |
| `public-testnet/` | Testnet maintainers | Public-testnet planning and proof records. | Proof Scaffold |
| `registry/` | Package/registry maintainers | Registry records and package discovery models. | Proof Scaffold |
| `renderer/` | Renderer maintainers | Renderer prototype/scaffold; not the canonical playable proof path yet. | Proof Scaffold |
| `reports/` | Audit/proof maintainers | Human-readable outputs from audits, validations, and certifications. | Documentation Only / Proof Scaffold |
| `runtime/` | Runtime maintainers | Contains the local runtime proof implementation and runtime-domain scaffolds. | Functional Prototype |
| `rustrigs/` | Tooling maintainers | Rustrig fixtures and executable proof support. | Experimental |
| `scripts/` | Developer/release tooling maintainers | Build, validation, certification, release, and proof scripts. | Functional Prototype / Proof Scaffold |
| `sdk/` | SDK maintainers | Older or alternate SDK-facing surfaces. | Experimental |
| `security/` | Security maintainers | Security records, models, and supporting material. | Proof Scaffold |
| `src-bin-everarcade/` | CLI/tooling maintainers | Source for binary/tooling experiments. | Experimental |
| `studio/`, `studio-gui/` | Creator tooling maintainers | Studio/editor prototypes. | Experimental |
| `templates/` | Creator experience maintainers | Shared game and runtime templates. | Functional Prototype |
| `test_vectors/`, `tests/` | Quality maintainers | Test fixtures and validation inputs. | Functional Prototype |
| `tools/` | Tooling maintainers | Utility tooling and repo support scripts. | Experimental |
| `vendor/` | Build/release maintainers | Vendored Cargo dependencies when complete. Current snapshot is incomplete. | Proof Scaffold |
| `wasm/` | Runtime contract maintainers | WASM fixtures and protocol proof support. | Functional Prototype |
| `world/` | World runtime maintainers | World runtime models and persistence fixtures. | Functional Prototype / Proof Scaffold |
| `xaman/`, `xrpl/`, `xrpl-anchor/` | Wallet/settlement maintainers | Wallet and settlement models. | Proof Scaffold |

## Current support boundary

The supported v0.1 contributor path is the Creator SDK + local runtime + replay verification path. Other subsystems should be documented, labeled, and tested honestly before being promoted beyond scaffold or experimental status.
