# Commercial Boundary Audit — 2026-06-26

## 1. Executive summary

This audit reviewed the public repository for material that may disclose EverArcade commercial strategy beyond the intended open-source scope: the `world.evr` artifact format, deterministic world generation, verification, attestation, fixtures, and reference implementation.

The repository remains useful as an open technical foundation, but it currently contains several commercial-adjacent surfaces that should be separated before broader public promotion. The highest-risk items are explicit revenue/treasury guides and records, hosted/control-plane scaffolds with cost assumptions, creator/GPU marketplace flows, and roadmap/export notes that describe future registry, marketplace, treasury, and badge rollout. Most findings are not secrets and many are labeled as non-production or deterministic fixtures; however, the quantity and specificity can still reveal future commercial product direction.

No code or documentation was removed as part of this audit. Recommended actions focus on moving private strategy to a private business repository, archiving historical planning exports, and generalizing public wording so the open repo stays centered on deterministic artifacts, local verification, attestation, trust roots, fixtures, and reference implementation.

## 2. Search commands run

The following commands were run from the repository root:

```bash
rg -n -i "registry|verification service|verified badge|vault|treasury|custody|operator|hosted|hosting|enterprise|integration|billing|subscription|pricing|take rate|revenue|tax|opportunity zone|QOZ|institutional|SLA|admin|dashboard|marketplace|certification|badge|world operator|Evernode lease|Xahau hook|XRPL hook|black.?hole|issuer|partner|Ripple|SBI|Anodos|Dane|Hugegreencandle|investor|valuation|exit|acquisition|commercial|proprietary|private" . -g '!target' -g '!vendor' -g '!node_modules' -g '!dist/everarcade-world-factory-release/**'
```

```bash
rg -n -i "revenue|pricing|billing|subscription|take rate|treasury|vault|custody|hosted|enterprise|partner|opportunity|institutional|verified badge|badge|SLA|admin|dashboard|marketplace|operator|evernode|xrpl|xahau|black.?hole|issuer" docs runtime tools frontend creator-sdk contract-api execution-core registry exports -g '!target' -g '!vendor' -g '!node_modules' -g '!dist/everarcade-world-factory-release/**'
```

```bash
find docs -maxdepth 3 -type f | rg -i "commercial|revenue|marketplace|treasury|vault|enterprise|operator|host|registry|certification|roadmap|founding|evernode|xrpl|xahau" | sort
```

```bash
find . -maxdepth 3 -type d | rg -i "marketplace|treasury|vault|registry|billing|enterprise|operator|control|frontend|revenue|evernode" | sort
```

```bash
find commercial-revenue creator-marketplace marketplace gpu/marketplace hooks/vault-hook hooks/registry-hook control-plane provider-evernode -maxdepth 2 -type f | sort
```

## 3. Findings table

| ID | Area | Files / examples | Classification | Risk | Finding | Recommended action | Required before further promotion? |
|---|---|---|---|---|---|---|---|
| CB-01 | Treasury / revenue | `docs/world-treasury-execution-layer.md`, `docs/treasury.md`, `docs/commercial-revenue/**`, `commercial-revenue/**` | Move Private / Generalize | High | Public docs and deterministic records describe revenue classes, contributor compensation, treasury health, operator compensation, settlement intents, marketplace fees, subscriptions, and revenue roots. Even when non-custodial and fixture-only, the combined surface reads like a future paid treasury/revenue product plan. | Move detailed revenue and treasury execution docs/records to a private business or product-planning repo. Keep only a short public note that implementations may attach non-authoritative economic metadata and that live payments/accounting are out of scope. | Yes |
| CB-02 | Hosted operator infrastructure | `control-plane/**`, `provider-evernode/**`, `docs/architecture/gpu_hosting_strategy.md`, `docs/runtime/evernode-*`, `docs/runtime-platform/live-evernode-lease-v0.1.md`, `templates/evernode/**` | Generalize / Move Private | High | The repo includes operator control-plane modules, Evernode lease lifecycle surfaces, runtime deployment/health/metrics concepts, and cost model assumptions. This may expose hosted operator platform assumptions and commercial infrastructure direction. | Keep generic local/operator reference specs. Move provider-grade automation, cost estimates, and hosted orchestration assumptions private until the hosting model is intentionally public. | Yes |
| CB-03 | Marketplace implementation and monetization signals | `creator-marketplace/**`, `creator-sdk/marketplace/**`, `gpu/marketplace/**`, `runtime/gpu-marketplace/**`, `docs/creator-marketplace/**`, `docs/runtime-platform/gpu-marketplace-v0.1.md`, `docs/capability-marketplace.md` | Generalize / Move Private | High | Creator and GPU marketplace records include discovery, licensing, royalties, usage tracking, settlement intent, reputation, provider registration, and reward units. This is more specific than a public protocol placeholder and can reveal marketplace product scope. | Retain protocol-neutral fixture examples only if clearly tied to deterministic artifact verification. Move royalty, usage, reputation, settlement, and provider marketplace details private or generalize as optional extension metadata. | Yes |
| CB-04 | Registry discovery strategy | `docs/registry/world-registry.md`, `docs/world-registry-spec-v1.md`, `registry/**`, `runtime/content-registry/**`, `contract-api/src/registry_validation/**`, `hooks/registry-hook/**` | Generalize | Medium-High | Public registry material includes discovery APIs, vitality sorting, reputation, contributor discovery, governance visibility, health, and abuse-resistance language. Some is safe as a public registry spec, but vitality/ranking/discovery details may reveal proprietary discovery model direction. | Keep `world.evr` registry schema, public protocol records, local fixtures, and validation. Generalize ranking/scoring language; remove or defer detailed discovery/reputation strategy. | Yes, for promotional materials that emphasize registry/discovery |
| CB-05 | Verification badge / certification rollout | `exports/2026-06-22-world-evr-v1-freeze/**`, `docs/world-package-certification-framework-v1.md`, `docs/world-contract-certification-framework-v1.md`, `tools/src/vertical_slice_certification.rs`, `tools/src/package_certification/**` | Keep Public / Generalize / Archive | Medium | Local package/world certification is within scope, but export notes mention public badge policy, portal integration, and World Verified badge thresholds. Badge rollout and application workflow may imply hosted verification service direction. | Keep local verifier, attestation, trust-root docs, and must-fail fixtures public. Archive historical export notes or generalize badge language to implementation-neutral verification metadata. | Recommended before broad promotion |
| CB-06 | Vault / custody / settlement hooks | `hooks/vault-hook/**`, `docs/runtime-platform/vault-ownership-certification.md`, `docs/architecture/xahau_xrpl_settlement_boundary.md`, `docs/xrpl-integration.md`, `runtime/xrpl-anchor-proof/**`, `frontend/shared-wallet/**` | Generalize / Keep Public if bounded | Medium | Vault hook and wallet/XRPL/Xahau materials are mostly scaffold or deterministic boundary docs, but names such as vault, settlement gateway, wallet authority, and future Xahau provider can be interpreted as custody architecture. | Add or preserve strong disclaimers: no custody, no private keys, no live settlement, no legal certification. Keep deterministic anchoring and payload-binding docs; move any custody architecture/private wallet flow private if added later. | Recommended |
| CB-07 | Hosted platform / dashboards | `frontend/creator-dashboard/**`, `frontend/operator-console/**`, `frontend/player-portal/**`, `frontend-gateway/**`, `player-gateway/**`, `docs/architecture/operational-readiness.md` | Generalize / Keep Public if demo-only | Medium | Dashboard and portal packages are named like SaaS/admin surfaces. Current evidence appears demo/status oriented and package manifests are private npm packages, but they could signal hosted platform direction. | Label these as demo/local/reference frontend surfaces. Move admin/customer/account/billing functionality private if it exists or is added. | No, unless promoted as product UI |
| CB-08 | Enterprise / partner / institutional strategy | Search terms including `enterprise`, `institutional`, `Ripple`, `SBI`, `Anodos`, `Dane`, `Hugegreencandle`, `opportunity zone`, `QOZ`, `tax`, `investor`, `valuation`, `acquisition` | Keep Public / No material finding | Low | Broad search did not surface obvious non-public partner names, tax-alpha strategy, investor materials, valuation, acquisition, or private business-development plans in the reviewed output. XRPL/Xahau ecosystem references are public technical compatibility context. | Continue to block partner-specific roadmaps, institutional pitch docs, legal/tax strategy, and non-public relationship assumptions from the public repo. | No |
| CB-09 | Historical roadmap/export bundles | `exports/2026-06-22-world-evr-v1-freeze/**`, `reports/**`, `archive/development-artifacts/**`, roadmap docs under `docs/architecture/roadmap/**` and `docs/release/**` | Archive / Generalize | Medium | Export and roadmap bundles contain useful engineering history but also mention staged registry expansion, capability marketplace growth, real treasury execution, portal verifier integration, Founding Worlds, and badge policy. | Move commercially sensitive historical bundles to an archive/private repo or add a public-boundary index explaining that they are historical, non-binding, and not product commitments. | Recommended |
| CB-10 | Secrets / accidental private files | Repository scan of searched terms and obvious commercial directories | Keep Public / Monitor | Low | No production API keys, wallet seeds, private keys, billing provider secrets, or non-public partner records were identified from the audit searches. One unrelated untracked `runtime/gpu-marketplace/Cargo.lock` was present before this task. | Add periodic secret scanning and boundary audit checks before public releases. Do not remove fixture keys unless they are not explicitly test-only. | No |

## 4. Recommended action

### Immediate public-boundary cleanup

1. Move detailed commercial revenue and treasury execution material out of the public repo or replace it with a short protocol-neutral economic-metadata boundary note.
2. Generalize marketplace material so public docs describe deterministic capability/package metadata and local fixtures, not royalty, usage tracking, provider reputation, reward units, or future settlement products.
3. Generalize registry discovery language by keeping schemas, fixtures, and validation while removing proprietary ranking/vitality/discovery strategy.
4. Archive or private-move historical export notes that describe staged commercial rollout, public badge policy, treasury execution, marketplace growth, or Founding Worlds strategy.
5. Label dashboards, portals, control-plane, Evernode, XRPL/Xahau, and hook directories clearly as local/reference/scaffold/demo unless and until they are intentionally released as product surfaces.

### Preserve without weakening

Do not remove or weaken:

- `world.evr` artifact format.
- Deterministic World Factory behavior.
- Local verifier and CLI verification.
- Trust-root and attestation models.
- Payload binding docs.
- Must-fail fixtures.
- Public reference implementation.
- Open-source README, release notes, and independent review history.

## 5. Risk level

Overall risk: **High before broad public promotion**.

The risk is not primarily accidental secret exposure. The risk is strategic disclosure: the repo contains enough revenue, treasury, marketplace, registry, hosted-operator, and badge-rollout detail to reveal a future commercial platform narrative beyond the intended open-source scope.

## 6. Whether action is required before further promotion

**Yes.** Before additional promotion, especially to developers, operators, commercial partners, or ecosystem institutions, complete at least a non-destructive cleanup pass that moves or generalizes high-risk treasury/revenue, marketplace, operator-hosting, and roadmap/export materials.

The open-source foundation can remain public and useful if commercial services are separated into private planning/product repositories and the public repo is framed around deterministic artifacts, local verification, attestation, fixtures, trust roots, and reference implementation.

## 7. Follow-up cleanup PR plan

1. **PR 1 — Boundary labels and index**
   - Add a repository-boundary index that defines public, scaffold, archived, and private-only categories.
   - Add clear disclaimers to scaffold directories that are safe to keep public.

2. **PR 2 — Revenue and treasury separation**
   - Move or replace detailed revenue/treasury docs and records.
   - Retain only generic, non-production economic metadata examples if needed for deterministic replay tests.

3. **PR 3 — Marketplace generalization**
   - Reduce creator/GPU marketplace docs to deterministic package/capability metadata and fixture examples.
   - Move royalty, usage tracking, reward, settlement-intent, provider reputation, and marketplace-growth strategy private.

4. **PR 4 — Registry/public discovery cleanup**
   - Keep schema, fixture, validation, and protocol language.
   - Generalize ranking, vitality, reputation, contributor discovery, and governance-curation specifics.

5. **PR 5 — Verification badge and historical archive cleanup**
   - Keep local verifier, attestation, and certification framework.
   - Archive or generalize World Verified badge policy, portal integration, threshold, and application workflow references.

6. **PR 6 — Hosted/operator/platform cleanup**
   - Label control-plane, Evernode, dashboard, portal, gateway, hook, and wallet integrations as local/reference/scaffold where appropriate.
   - Move production hosting, lease orchestration, cost model, admin/account/billing, and private endpoint assumptions to private repositories.

7. **PR 7 — Release guardrails**
   - Add a repeatable commercial-boundary audit checklist.
   - Add secret scanning and broad sensitive-term search commands to release certification docs.
