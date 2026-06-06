# Abuse and Adversarial Analysis

**Date:** 2026-06-05  
**Scope:** repository-level abuse review for the current EverArcade prototype/scaffold mix.  
**Conclusion:** abuse controls are not production-ready.

## Core trust-boundary problem

The repo contains many deterministic records and model roots. Those are useful for reproducibility, but a public deployment would need to prevent users, creators, operators, GPU providers, marketplace actors, and wallet/settlement participants from presenting self-authored records as authoritative truth.

## Abuse scenarios

### Economic abuse

- Attackers may forge or replay revenue `.records` files as if they were payment obligations.
- Creator/operator/provider revenue roots can be spammed or selectively omitted.
- Demo-only monetization records may be misrepresented as billable production activity.

**Required before launch:** signed ledger-backed settlement, accounting controls, dispute handling, rate limits, audit trails, and explicit non-authoritative labels on revenue scaffolds.

### Marketplace abuse

- Fake listings, wash trades, counterfeit assets, and duplicate creator packages can be represented in deterministic records.
- Marketplace records do not prove ownership, settlement finality, fraud checks, refunds, or takedowns.

**Required before launch:** verified asset lineage, creator identity controls, listing moderation, payment finality checks, escrow/dispute rules, and spam controls.

### GPU provider fraud

- Providers can claim device capacity or completed render artifacts without actually performing work.
- Render output can be cherry-picked, fabricated, or computed with nondeterministic settings.

**Required before launch:** challenge/response jobs, reproducible render verification, provider reputation, stake/slashing or withholding, hardware attestation if claimed, and anti-Sybil controls.

### Fake render artifacts

- Renderer/GPU artifacts are non-authoritative projections and can be fabricated.
- A fake artifact may be used to deceive players or claim provider rewards.

**Required before launch:** bind artifacts to authoritative projection roots, checkpoint/replay ids, renderer version, deterministic settings, and independent verification.

### Settlement spoofing

- XRPL-shaped receipts and payment transaction records can be generated without live ledger finality.
- Continuity anchors can be confused with actual ledger settlement.

**Required before launch:** live ledger lookup, transaction hash verification, finality depth policy, account/currency/amount validation, replay protection, and chain/network id binding.

### Xaman payload spoofing

- Payload IDs, request records, QR/deep-link metadata, and imported signed receipt records can be copied or invented.
- A UI could treat a deterministic payload record as wallet approval.

**Required before launch:** Xaman API verification, payload expiry, user binding, anti-replay nonce, callback signature checks, and clear key-custody boundaries.

### Replay divergence

- Divergent replay streams can be hidden behind PASS reports or partial roots.
- Different layers may accept different canonical orderings or hashes.

**Required before launch:** canonical replay format, cross-machine replay gates, signed replay manifests, divergence alarms, and quarantine of conflicting roots.

### Checkpoint poisoning

- Malicious checkpoints can be imported to rewrite state, skip invalid history, or seed bad world metadata.
- Federation checkpoint exchange records do not by themselves prove quorum acceptance.

**Required before launch:** checkpoint signatures, quorum/finality rules, monotonic chain validation, world-id/runtime-version binding, and rollback/dispute procedures.

### Malicious game templates

- Creator templates can contain arbitrary JavaScript/package scripts.
- Templates can exfiltrate local files, run postinstall scripts, or confuse users into trusting generated artifacts.

**Required before launch:** sandboxed template execution, dependency policy, template signing, static scanning, minimal permissions, and no implicit execution during project creation.

### Creator marketplace spam

- Automated creators can flood records, templates, modules, and publications.
- Similar generated content can drown legitimate packages.

**Required before launch:** identity verification tiers, quotas, proof-of-work/stake or review queues, duplicate detection, reputation, and takedown workflows.

### Player identity abuse

- Player profile/session records can be forged, replayed, or linked across contexts.
- Resume tokens and local session markers can be leaked if treated casually.

**Required before launch:** authenticated session service, token rotation, scoped identity, privacy policy, wallet binding rules, and rate limits.

### Operator misbehavior

- Operators can censor inputs, reorder records, withhold checkpoints, equivocate across players, or publish fake health.
- Appliance status files alone are not a trust anchor.

**Required before launch:** operator attestations, independent observers, slashing/dispute procedures, public audit logs, and quorum validation for critical state.

### Denial of service

- Script surfaces, runtime package loading, template creation, gateway records, and replay ingestion can be abused with large files, many records, or malformed JSON.
- Hundreds of scripts increase attack and maintenance surface.

**Required before launch:** input limits, file-size caps, CPU/memory quotas, backpressure, JSON schema validation, authenticated APIs, and CI limits.

### Supply-chain risks

- Node templates and CLI scripts execute local JavaScript.
- Cargo is configured for vendored offline dependencies, but this clone's vendor source was incomplete for targeted runtime build.
- `node_modules/` is present in the repo tree and can obscure dependency policy.

**Required before launch:** lockfile policy, complete vendoring or no vendoring, dependency scanning, provenance, reproducible builds, and no committed dependency directories unless intentionally documented.

### Open-source exploitation risks

- Public users may mistake scaffold records for live settlement, revenue, marketplace, or wallet systems.
- Attackers can use public scripts/docs to craft convincing fake PASS artifacts.
- Incomplete security policies and unclear support boundaries increase disclosure risk.

**Required before launch:** explicit maturity labels, security policy, supported commands, threat model, abuse-report process, and removal or quarantine of misleading PASS artifacts.

## Highest-priority mitigations

1. Label every scaffold/model/report as non-authoritative unless proven live.
2. Create a single supported local launch command and make it produce real runtime/session/projection evidence.
3. Add SECURITY.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, and LICENSE.
4. Fix vendored dependency completeness or document network build mode.
5. Add schema validation and size limits to all JSON/record ingestion.
6. Separate production claims from certification/demo claims.
