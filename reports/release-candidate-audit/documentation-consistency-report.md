# Documentation Consistency Report

## Required documents checked

| Document | Status | Notes |
|---|---:|---|
| README | PASS | Presents clone/read/create/run/package/verify flow. |
| Quick Start | PASS | CLI quick-start docs exist; README now points to first-world path. |
| First World | PASS | `docs/first-world.md` exists and is the intended onboarding bridge. |
| Creator SDK | PASS | `creator-sdk/README.md` documents `everarcade world ...`. |
| Operator Guide | PASS | Runtime/operator docs exist; not primary creator flow. |
| World Package Guide | PASS | World package docs exist and align with package terminology. |
| Template Library | PASS | Template library docs exist and CLI lists templates. |
| RustRig Docs | PASS | RustRig docs exist and CLI lists certified/candidate rigs. |

## Terminology

Canonical terms for v0.1 public docs are: World, World Package, World Contract, RustRig, Projection, and Deployment.

Observed inconsistent/legacy wording:

- Older Creator SDK root markers still mention `everarcade new -> build -> test -> deploy -> publish`.
- Some docs and examples use game/package wording where public onboarding now says World and World Package.
- Runtime proof directories use HotPocket, federation, renderer, and public-testnet language that must remain explicitly experimental/scaffold-level.

## Decision

KEEP existing deeper docs, but public entry points should continue to prefer the World workflow and link experimental domains through maturity/readiness warnings.
