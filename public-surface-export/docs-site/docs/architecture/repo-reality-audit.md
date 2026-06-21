# Repository Reality Audit

**Date:** 2026-06-05  
**Overall result:** **NOT PRODUCTION READY**

EverArcade currently reads as a broad deterministic runtime/platform prototype plus many certification scaffolds. The repository contains real Rust and Node code, but also many records, placeholder appliance directories, shell models, and PASS reports that do not prove live production behavior.

## Classification summary

| Area | Classification |
| --- | --- |
| execution-core | Functional Prototype |
| everarcade-runtime | Functional Prototype |
| runtime appliances | Functional Prototype |
| node appliance | Certification Scaffold |
| HotPocket layer | Certification Scaffold |
| Evernode layer | Functional Prototype |
| XRPL layer | Certification Scaffold |
| Xaman layer | Certification Scaffold |
| federation | Certification Scaffold |
| renderer | Certification Scaffold |
| GPU runtime | Certification Scaffold |
| GPU marketplace | Certification Scaffold |
| developer portal | Certification Scaffold |
| creator SDK | Functional Prototype |
| game templates | Functional Prototype |
| player gateway | Certification Scaffold |
| game discovery | Certification Scaffold |
| commercial revenue | Certification Scaffold |
| scripts | Functional Prototype / Obsolete Duplicate |
| docs | Documentation Only / Unclear |
| reports | Documentation Only / Certification Scaffold |

## What appears real

- `execution-core` has substantial Rust source breadth and a large test tree.
- `runtime/everarcade-runtime` has a concrete Rust operator/runtime loop for package loading, status, journals, receipts, checkpoints, backup, and replay reporting.
- `creator-sdk/cli/everarcade.mjs` can create, build, test, deploy, and publish local JSON artifacts for templates.
- Several package/build/runtime scripts provide real local automation paths, subject to dependency and layout assumptions.

## What is scaffolded or model-only

- Renderer, federation, GPU marketplace/runtime records, player gateway, game discovery, commercial revenue, XRPL, Xaman, HotPocket, and node appliance directories are mostly deterministic records, README files, shell models, fixtures, and filesystem placeholders.
- Existing PASS reports should be read as model/certification evidence unless the corresponding script proves external live execution.
- Revenue, settlement, wallet, GPU provider, marketplace, and gateway claims are not production systems.

## What is confusing

- The repository mixes runnable crates, shell models, generated records, reports, documentation, frontend samples, appliance layouts, and placeholder directories without a single authoritative repo map.
- There are hundreds of scripts with overlapping `run_*`, `validate_*`, `verify_*`, and `certify_*` names.
- Several terms recur across different layers: runtime, node, appliance, gateway, renderer, replay, federation, settlement, marketplace, and portal.
- Some documentation implies launch/readiness while underlying directories are records or placeholders.

## Production readiness conclusion

The repo should not be presented as a complete production platform. It can be presented as a pre-production deterministic runtime/compiler prototype with explicit scaffold boundaries and known gaps.
