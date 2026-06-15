# Repository Completion Assessment

Audit date: 2026-05-30

These maturity percentages are planning estimates, not product guarantees. They separate a **validated model** and **working local CLI/runtime** from a **real live deployment** or **real production backend**.

| Area | Maturity estimate | Current evidence | Boundary / caveat |
| --- | ---: | --- | --- |
| Deterministic runtime | 72% | Broad execution-core deterministic, replay, receipt, state, WASM, and creator runtime tests; targeted offline validation gates. | This is a validated local/model runtime, not yet a production service SLO. |
| Replay/federation | 48% | Many replay/federation tests and validation scripts exist. | Treat federation/history/live networking as scaffold-level until multi-node live deployments are proven outside tests. |
| World/civilization runtime | 52% | World, economy, civilization, governance, and persistent runtime tests exist across execution-core and host surfaces. | Model coverage is broad but production persistence/ops readiness remains unproven. |
| Simulation/ECS/AI | 45% | Simulation runtime, ECS, AI, ecology, faction, and entity reports/scripts exist. | Validated scaffold plus internal runtime models; not an end-user simulation platform yet. |
| Developer SDK | 42% | SDK crates exist and SDK validation checks deterministic API files. | API contract, versioning, docs, and external developer guarantees need hardening. |
| Studio GUI | 38% | Studio package tests and launch-readiness creator workflow filters pass targeted validation. | Working local workflow logic; not yet polished product UX. |
| Creator workflow | 60% | Creator pipeline, template generation, play mode, package generation, publish workflow, readiness, and replay-safe pipeline tests exist. | Best current end-to-end internal surface; still not a real marketplace/deployment backend. |
| Vertical slice | 64% | Vertical slice certification, package reproducibility, and new-developer success metric tests pass. | Validates the intended local path, not broad product completion. |
| EverNode deployment readiness | 30% | EverNode manifest/adapter/report surfaces and deployment scripts exist. | Mostly validated model/scaffold; no real live EverNode deployment was performed. |
| XRPL anchoring readiness | 28% | XRPL anchor/settlement tests and docs/report surfaces exist. | Anchoring readiness is modeled; real network anchoring and operations are not certified here. |
| Fresh VM reproducibility | 70% | Vendor regeneration, offline locked cargo metadata, and targeted offline locked tests are used. | Ready for another fresh-VM validation attempt; full workspace remains intentionally unrun and likely sensitive. |

## Explicit capability distinctions

- **Validated model:** Deterministic execution, creator pipeline, protocol readiness, launch-readiness filters, security tests, and runtime surface classifications are validated by local/offline tests and scripts.
- **Working CLI:** CLI surfaces and aliases exist, but this audit did not certify a full CLI user journey as launch-critical.
- **Working local runtime:** Local deterministic runtime and creator vertical-slice paths are the strongest validated areas.
- **Real live deployment:** Not proven in this pass. EverNode, XRPL, live federation, networking, and production ops remain scaffold/model readiness.
- **Real production backend:** Not present/certified. Reports and tests should not be interpreted as a deployed production control plane.

## Overall assessment

The repository is best described as a broad, test-rich deterministic runtime and creator-platform prototype with a usable internal vertical slice. Fresh-VM reproducibility is materially improved by vendoring and targeted tests, but the repository still contains many scaffold-level runtime, federation, renderer, deployment, and operations domains.
