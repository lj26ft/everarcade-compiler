# Human-Readable EverArcade Repo Map

**Purpose:** explain what each major area is, who should care, and whether it is real implementation or scaffold.

## Start here

1. `README.md` — high-level project maturity and quickstart.
2. `docs/stub-vs-usable-matrix.md` — intended source of truth for stub vs usable areas.
3. `docs/architecture/repo-reality-audit.md` — this audit's production-readiness classification.
4. `reports/local_game_launch_report.txt` — what the arena local game flow actually proved.

## Core implementation

- `execution-core/` — broad Rust protocol/runtime model. Treat as functional prototype unless a specific module has targeted tests and integration docs.
- `runtime/everarcade-runtime/` — small Rust appliance runtime with package validation, runtime status, journals, receipts, checkpoints, backup, and replay reports.
- `everarcade-abi/`, `everarcade-host/`, `contract-api/`, `control-plane/`, `provider-evernode/` — supporting Rust crates and provider surfaces.

## Creator and game surfaces

- `creator-sdk/` — Node CLI plus templates. It creates local JSON artifacts; it does not create production runtime packages.
- `creator-examples/` and `examples/` — sample/project surfaces of mixed maturity.
- `creator-marketplace/` and `marketplace/` — marketplace records/scaffolds, not production commerce.
- `arena-vanguard*` — specialized arena sample/package/browser/gateway/host/world directories. Ownership overlaps with templates and frontend samples.

## Runtime/platform scaffolds

- `runtime/` — umbrella for many runtime domains. Some subdirectories are crates; many are status records, scripts, reports, or placeholders.
- `node/` — appliance filesystem layout placeholder.
- `evernode/` — Evernode appliance layout placeholder; `provider-evernode/` contains the more concrete Rust provider prototype.
- `hotpocket/` — adapter/model directories for HotPocket integration, not a live deployment.
- `federation/` — deterministic evidence-exchange model and fixtures. Treat as scaffold-level.

## Projection and acceleration

- `renderer/` — non-authoritative projection model. Treat as scaffold-level.
- `gpu/` — GPU job/device/worker/artifact/verification records and models.
- `runtime/gpu-runtime/` and `runtime/gpu-marketplace/` — prototype crate surfaces, not proven production GPU provider marketplace.

## Player/developer product surfaces

- `player-gateway/` — player-facing records and gateway taxonomy, not a production authenticated gateway.
- `developer-portal/` — developer portal records/model, not a production portal service.
- `game-discovery/` — catalog/discovery records, not a live search/discovery service.
- `frontend/`, `frontend-gateway/`, `clients/` — frontend and gateway experiments/samples.

## Settlement and wallet boundaries

- `xrpl/` — deterministic XRPL-shaped settlement artifacts. No live signing/submission/finality.
- `xaman/` — deterministic Xaman payload/status/receipt artifacts. No live API/key custody.
- `hooks/` — hook-like directories; do not assume deployed hooks without explicit deployment evidence.

## Commercial surfaces

- `commercial-revenue/` — deterministic revenue records and roots. Not billing, payout, tax, marketplace settlement, or accounting software.
- `public-testnet/` — public testnet records/scaffold, not evidence of a live public network unless accompanied by current endpoints and verification steps.

## Automation and evidence

- `scripts/` — large automation surface. Prefer documented supported scripts; many scripts are certification or model runners.
- `reports/` — audit/certification/report outputs. Reports must say whether they are live, simulated, dry-run, or documentation-only.
- `docs/` — broad documentation set. Claims should be checked against code and scripts before public use.

## Rule of thumb

If a directory mostly contains README files, `.records`, root marker files, `.gitkeep`, fixtures, or shell root-generation models, classify it as scaffold/certification until proven otherwise by a runnable command and integration test.
