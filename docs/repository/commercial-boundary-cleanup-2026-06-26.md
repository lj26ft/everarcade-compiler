# Commercial Boundary Cleanup — 2026-06-26

## Scope

This cleanup reviewed the public repository for material that could expose future commercial strategy around registry services, vaults, treasuries, hosted operators, marketplaces, enterprise products, pricing, institutional plans, or paid verification services.

The public repository boundary is now stated in [`REPOSITORY_BOUNDARY.md`](../../REPOSITORY_BOUNDARY.md): this repo explains how `world.evr` works and preserves deterministic generation, verification, attestations, trust-root validation, payload binding, fixtures, and reference examples.

## Files scanned

The cleanup used the requested commercial-boundary ripgrep scans across the repository, excluding `target`, `vendor`, `node_modules`, and `dist` where applicable. The manual review focused on these high-risk areas:

- Treasury/revenue docs and records: `docs/treasury.md`, `docs/world-treasury-execution-layer.md`, `docs/commercial-revenue/**`, `commercial-revenue/**`, revenue-related reports and scripts.
- Marketplace material: `creator-marketplace/**`, `creator-sdk/marketplace/**`, `marketplace/**`, `gpu/marketplace/**`, `runtime/gpu-marketplace/**`, `docs/creator-marketplace/**`, `docs/runtime-platform/gpu-marketplace-v0.1.md`, `docs/capability-marketplace.md`.
- Hosted operator/control-plane material: `control-plane/**`, `provider-evernode/**`, `templates/evernode/**`, `deployment/evernode/**`, `docs/runtime/evernode-*`, `docs/runtime-platform/live-evernode-lease-v0.1.md`, `docs/architecture/evernode-integration.md`.
- Registry material: `docs/registry/**`, `docs/world-registry-spec-v1.md`, `registry/**`, `runtime/content-registry/**`, `hooks/registry-hook/**`, reference registry examples.
- Verification/certification material: `specs/world-evr-package/**`, `docs/world-package-certification*.md`, `docs/world-contract-certification-framework-v1.md`, `tools/src/vertical_slice_certification.rs`, certification reports and fixtures.
- Vault/custody/settlement material: `hooks/vault-hook/**`, `docs/runtime-platform/vault-ownership-certification.md`, `docs/architecture/xahau_xrpl_settlement_boundary.md`, `docs/xrpl-integration.md`, `xrpl/**`, `runtime/xrpl-anchor-proof/**`, `frontend/shared-wallet/**` when present.
- Enterprise/partner/institutional terms across docs, scripts, reports, examples, and source.

## Files changed

- `REPOSITORY_BOUNDARY.md` was added to define the public open-source scope and out-of-scope commercial surfaces.
- `README.md` now links to the repository boundary document from the documentation index.
- `docs/repository/commercial-boundary-cleanup-2026-06-26.md` records this cleanup, reviewed areas, and remaining risks.
- `commercial-revenue/**` was removed from the public repository.
- `docs/commercial-revenue/**` was removed from the public repository.

## Material removed

Detailed commercial revenue records and guides were removed from `commercial-revenue/**` and `docs/commercial-revenue/**`. No `world.evr` source implementation, fixtures, RC1/RC2 review history, verifier behavior, or reference examples were removed. Other high-risk terms were handled non-destructively because many appear in deterministic local scaffolds, historical certification reports, or protocol/reference test material that may be used by existing validation gates.

## Material generalized

The newly added repository boundary generalizes the public position for these areas:

- Economic and treasury systems are out of scope except for non-authoritative metadata examples.
- Marketplace operation, royalties, settlement, reputation, rewards, and hosted marketplace services are out of scope.
- Hosted operator files are local/reference scaffolds, not a production hosted platform, SLA service, or commercial operator network.
- Registry material is limited to neutral identity, metadata, and local validation examples.
- Verification/certification remains local and primitive-focused; hosted badge programs, paid workflows, and reviewer marketplaces are out of scope.
- XRPL/Xahau material is limited to deterministic anchoring, boundary modeling, or local test scaffolds and does not provide custody, wallet management, live settlement, or legal ownership certification.
- Enterprise integrations and commercial partnerships are out of scope; only public technical compatibility belongs here.

## Material intentionally kept

The following categories were intentionally preserved:

- `world.evr` artifact format and reference examples.
- Deterministic World Factory behavior and local verifier/CLI verifier paths.
- Attestation, payload binding, trust-root validation docs, and must-fail fixtures.
- RC1/RC2 independent review history and public release notes.
- Local deterministic scaffolds whose names include terms such as registry, marketplace, treasury, settlement, vault, hooks, metrics, or operator when those files are part of reference validation, compatibility modeling, historical certification evidence, or scaffold-level runtime domains.
- Public technical compatibility references to XRPL/Xahau/Evernode where framed as anchoring, boundary modeling, local test scaffolds, or non-production runtime experiments.

## Remaining follow-up risks

- The repository still contains many historical scaffold directories and generated records whose filenames include commercial terms. The boundary scan intentionally reports these so maintainers can decide whether to archive entire historical surfaces privately in a later, broader cleanup.
- Generated reports under `reports/**` may contain old wording from previous certification runs. They were not rewritten to avoid altering historical evidence without a dedicated archival policy.
- Source modules in scaffold-level runtime domains still use names such as marketplace, treasury, settlement, vault, control plane, provider, and registry. They should remain acceptable only while documented as local/reference or deterministic modeling code, not commercial services.
- If this repository is prepared for a smaller public distribution, consider a follow-up export allowlist that includes only `world.evr`, verifier, trust-root, attestation, payload-binding, fixtures, release notes, and reference examples.

## Remaining scan classification

Remaining matches from the final commercial-boundary scan are classified as follows:

- Acceptable public protocol/reference material: `world.evr` docs, trust-root/attestation docs, local verifier code, reference registry examples, RC1/RC2 bundles, must-fail fixtures, and public release notes.
- Generalized language: `README.md`, `REPOSITORY_BOUNDARY.md`, and this cleanup report now state the commercial boundary explicitly.
- Needs private removal: creator marketplace records, hosted operator deployment scaffolds, and settlement/custody-adjacent reports should be considered for private archival if the public repo is narrowed further; detailed commercial revenue guides and records have been removed from this public tree.
- False positives: common technical words such as hook, rank, score, metrics, fee in generic contexts, tax in dependencies or legal boilerplate, and registry in neutral schema/validation contexts.
