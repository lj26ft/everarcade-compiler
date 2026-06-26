# Open Source Repository Audit & Cleanup Plan (2026-06-26)

This audit reviews the repository from the perspective of a first-time open source contributor. It intentionally avoids destructive cleanup: the recommendations below should be reviewed before files are deleted or moved. The proposed direction favors preserving historical engineering work in `archive/` over losing context.

## Scope and method

Commands used during the audit:

- `find .. -name AGENTS.md -print`
- `git status --short`
- `find . -path './node_modules' -prune -o -path './vendor' -prune -o -path './target' -prune -o -type f`
- `git ls-files`
- `git ls-files -z | xargs -0 du -h | sort -hr | head -40`
- `git ls-files | rg -i '(private|secret|token|key|pem|cert|credential|password|ghp)'`
- `find scripts .github/workflows -maxdepth 2 -type f`
- `find . -path './node_modules' -prune -o -path './vendor' -prune -o -type d \( -iname '*fixture*' -o -iname '*fixtures*' \)`

No build or verification behavior was changed as part of this audit.

## Executive summary

The project already has strong contributor-facing entry points: `README.md`, `MATURITY.md`, `REPOSITORY_MAP.md`, `CONTRIBUTING.md`, and `docs/README.md` explain the current maturity boundary and canonical documentation set. The largest open-source-maintenance risk is not missing documentation; it is the volume of historical, generated, review, report, fixture, and release-candidate material mixed with source code and canonical docs.

Top recommendations:

1. Move RC1/RC2 review bundles and root-level RC/proof notes into `archive/open-source-candidates/` after confirming no CI paths depend on their current locations.
2. Move generated review exports, old validation reports, and release reports out of primary contributor paths unless they are active fixtures.
3. Decide whether committed release bundles such as `dist/vendor.tar.gz` are intentionally source-of-truth reproducibility inputs or should move to GitHub Releases.
4. Introduce a top-level `docs/repository/` maintenance section for audits, repository maps, and cleanup plans so root markdown stays focused.
5. Document script responsibilities and consolidate overlapping `certify_*`, `run_*_validation`, and release packaging scripts through a manifest or index before removing any scripts.
6. Treat private keys under fixture/review paths as public test material only if they are clearly documented as non-secret test keys; otherwise replace with generated-at-test-time material.

## Findings by category

### Safe to remove after review

| Finding | Evidence | Recommendation | Justification |
| --- | --- | --- | --- |
| Generated release checksum without matching tracked archive | `dist/everarcade-world-factory-release.tar.gz.sha256` is tracked while `.gitignore` states CI produces `dist/everarcade-world-factory-release.tar.gz`. | Remove from Git after verifying no tests assert the checksum file. Keep the generated release bundle in GitHub Releases or CI artifacts. | A checksum for an absent generated archive is confusing for contributors and likely stale. |
| Generated runtime/package outputs in example `dist/` trees | `examples/adapter-demo-arpg/dist/certification/...` and `examples/world-factory/frontier-settlement/out/world.evr/...` appear as generated output paths. | Remove or move to fixture-specific directories only if used by tests; otherwise regenerate during validation. | `dist/` and `out/` conventionally imply generated outputs. Keeping them at source paths weakens the source/artifact boundary. |
| Untracked standalone crate lockfile | `runtime/gpu-marketplace/Cargo.lock` is currently untracked. | Do not add unless this crate is intentionally independently released; otherwise add a targeted ignore or delete locally. | The workspace already has root lockfile ownership; untracked generated locks should not surprise contributors. |
| Temporary/local text notes already ignored | `.gitignore` excludes several named VM/token text files and `*.txt`. | Keep ignored; remove any matching local files if present. | The ignore entries imply prior accidental local-note risk and should continue to protect future commits. |

### Archive

| Finding | Evidence | Recommendation | Justification |
| --- | --- | --- | --- |
| Root-level open-source candidate RC documents | `OPEN_SOURCE_CANDIDATE_RC1.md`, `OPEN_SOURCE_CANDIDATE_RC2.md`, and `OPEN_SOURCE_READINESS.md` live at repository root. | Move reviewed historical RC docs to `archive/open-source-candidates/rc1/` and `archive/open-source-candidates/rc2/`; keep only a short root link if needed. | Valuable history, but root should guide current contributors rather than present old release-candidate gates as current flow. |
| RC review bundles at top level | `OPEN_SOURCE_CANDIDATE_RC1_REVIEW_BUNDLE/` and `OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE/` are top-level directories. | Archive under `archive/open-source-candidates/` after updating any README/CI references. | They are self-contained historical review packages and create top-level clutter. |
| Root-level proof/trust docs | `GATE_PROOFS.md`, `PAYLOAD_BINDING.md`, `REVIEW_TRUST_CHAIN.md`, `TRUST_ROOT.md`, `ATT_V0_1_REFERENCE_IMPLEMENTATION.md`, `ATT_V0_1_REVIEW_HISTORY.md`, and `WORLD_RELEASE_ATTESTATION_V0_1_FREEZE.md`. | Move stable specs to `specs/` and historical review/proof notes to `archive/open-source-candidates/` or `docs/proofs/`. | These names are important but ambiguous at root; contributors need to know which are active specs versus historical proof notes. |
| Existing report corpus | `reports/` and `archive/development-artifacts/deployment-reports/` contain many validation and certification reports. | Keep in archive if used as evidence; otherwise add a dated index and move stale active reports into `archive/development-artifacts/`. | Reports are evidence, not onboarding material. They should not compete with canonical docs. |
| Review exports | `exports/world-release-attestation-*` and `exports/operator-identity-*` appear to be review packages and failure fixtures. | Archive or move under `fixtures/review/` if tests use them. | The `exports/` name reads as generated output; active test fixtures need an explicit fixture location. |

### Keep

| Finding | Evidence | Recommendation | Justification |
| --- | --- | --- | --- |
| Core contributor entry points | `README.md`, `CONTRIBUTING.md`, `MATURITY.md`, `REPOSITORY_MAP.md`, and `docs/README.md`. | Keep at root/docs root and ensure all historical links point back to these. | These are useful for the first five minutes of contributor onboarding. |
| Offline vendor bundle, if intentional | `.gitignore` explicitly unignores `dist/vendor.tar.gz` and `dist/vendor.tar.gz.sha256`; `README.md` documents no-network-after-clone behavior. | Keep only if maintainers intentionally trade repository size for offline reproducibility. If not, move to GitHub Releases and update onboarding. | The bundle is large but appears intentional. Removing it would change contributor/build behavior. |
| Vendored source tree, if restored from bundle or intentionally committed | `vendor/` is ignored but has tracked files, including nested AGENTS instructions. | Keep until the offline dependency model is redesigned. | Removing it could break offline validation. |
| Scaffold domains | Renderer, history, federation, XRPL, GPU marketplace, and public-testnet areas are already called out as scaffold/experimental in onboarding docs. | Keep, but maintain maturity labels and avoid presenting them as production-ready. | This preserves future-facing work while limiting contributor confusion. |

### Rename

| Finding | Evidence | Recommendation | Justification |
| --- | --- | --- | --- |
| Mixed naming styles for world factory | Paths and docs use `world-factory`, `World Factory`, and command names with `world`. | Prefer kebab-case for paths/docs (`world-factory`) and Rust/module identifiers in snake_case (`world_factory`). | Clear convention reduces search friction. |
| Trust root variants | Root files use `TRUST_ROOT.md`; fixtures use `trust-root`; code/docs likely use `trust_root`. | Keep filesystem directories kebab-case, code identifiers snake_case, and docs titles human-readable. Document this in `docs/naming.md`. | Avoids implying different concepts when only casing differs. |
| Verification terms | `verify`, `verification`, `certify`, and `certification` appear across docs/scripts. | Reserve `verify` for deterministic artifact checks and `certify` for release/readiness gates. Add a script naming note. | Helps contributors pick the correct command without reading every script. |

### Reorganize

| Finding | Evidence | Recommendation | Justification |
| --- | --- | --- | --- |
| Root has many markdown documents | Root markdown includes current onboarding, RC, proof, trust, maturity, and readiness docs. | Keep only project entry points at root: `README.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `LICENSE`, `MATURITY.md`, and `REPOSITORY_MAP.md`. Move the rest to `docs/`, `specs/`, or `archive/`. | A smaller root lowers first-time contributor cognitive load. |
| Documentation has both canonical docs and historical reports | `docs/README.md` says numbered docs are canonical and older docs are evidence. | Add `docs/archive/README.md` and `docs/repository/README.md` indexes, then gradually move older evidence under archive with redirects/links. | Makes the canonical/historical boundary visible in directory layout. |
| Test fixtures are spread across many subsystems | Fixture directories exist under `fixtures/`, `proofs/`, `specs/`, `exports/`, `execution-core/tests`, `everarcade-host/tests`, and examples. | Establish fixture categories: `positive/`, `must-fail/`, `regression/`, and `review/` within each subsystem or a central `fixtures/README.md`. | The requested fixture grouping will make test intent clearer without changing test behavior. |
| CI scripts are numerous and overlapping | `scripts/` contains many `certify_*`, `run_*_validation`, packaging, deployment, and operator scripts. | Add `scripts/README.md` or `scripts/MANIFEST.md` classifying scripts by lifecycle: onboarding, CI gates, release packaging, deployment/operator, archive-only. | Consolidation should start with documentation to avoid deleting a still-used operator script. |
| Release artifact locations are mixed | `dist/`, `release/`, `deployment/evernode/runtime/`, and generated `.tar.gz`/`.sha256` paths coexist. | Define one policy: source repo keeps reproducibility inputs and small fixtures; GitHub Releases keep generated release archives. Update `.gitignore` accordingly. | Reduces accidental publication of build products. |

## Security review notes

- The main concern found by filename scan is committed private-key-looking material in review/fixture paths, including `OPEN_SOURCE_CANDIDATE_RC2_REVIEW_BUNDLE/fixtures/trust-root/test-attester-private-key.pem` and `examples/world-factory/frontier-settlement/fixtures/attester-ed25519-private.pem`.
- If these are intentionally public test keys, add prominent `README.md` notes in each containing directory: "public test key, never used for production or deployment." If not intentional, replace with deterministic generated test material at validation time.
- `.gitignore` already contains protections for logs, temporary files, generated state, VM notes, and token-looking local notes. Keep these protections and consider replacing broad `*.txt` with targeted paths if legitimate text fixtures are expected in the future.
- No action was taken on keys in this audit because deleting or rotating fixture keys may alter verification behavior.

## Release artifact policy recommendation

Repository should contain:

- source code;
- specs;
- examples;
- small deterministic fixtures;
- tooling;
- explicit reproducibility inputs required by offline onboarding.

GitHub Releases or CI artifacts should contain:

- generated release bundles;
- packaged `.tar.gz` artifacts;
- generated runtime distributions;
- generated validation archives;
- checksums/signatures for generated artifacts unless they are part of a source-controlled reproducibility contract.

Special case: `dist/vendor.tar.gz` appears intentional because contributor onboarding advertises no-network-after-clone behavior. Treat it as an explicit exception until the offline dependency policy changes.

## Open-source first impression

A new contributor can understand the project within five minutes if they start at `README.md`, but the repository root and top-level directory list are intimidating. The README does a good job stating maturity and canonical gates. The friction starts after cloning: many top-level historical/review/generated-looking directories make it hard to distinguish active code from evidence.

Recommended onboarding improvements:

1. Add a one-page `docs/repository/where-things-live.md` that distinguishes source, fixtures, generated artifacts, historical archives, and scaffold domains.
2. Link that page from `README.md` and `REPOSITORY_MAP.md`.
3. Move old RC material into `archive/` with an index and a root-level pointer only if still important.
4. Add `scripts/MANIFEST.md` so contributors know which scripts are safe day-one checks.
5. Add fixture README conventions for positive, must-fail, regression, and review fixtures.

## Ordered cleanup checklist

Review and execute in this order to avoid CI or verification regressions:

1. **Inventory references before moves**: use `rg` to find every reference to RC bundles, root proof docs, `exports/`, `dist/`, and generated example outputs.
2. **Add indexes before moving**: create `archive/open-source-candidates/README.md`, `docs/repository/README.md`, and `scripts/MANIFEST.md`.
3. **Archive RC bundles**: move RC1/RC2 review bundles and root RC docs into archive; leave compatibility links or update references.
4. **Classify root proof docs**: move active specs to `specs/` or `docs/proofs/`; move historical proof notes to archive.
5. **Clarify public test keys**: add non-secret fixture notices or regenerate test keys during validation.
6. **Normalize fixture layout**: add fixture indexes and gradually split fixtures into positive, must-fail, regression, and review groups without changing test paths until references are updated.
7. **Document scripts**: write the script manifest; only then consolidate overlapping scripts.
8. **Release artifact cleanup**: remove stale generated checksums/artifacts and update `.gitignore` exceptions after confirming offline onboarding remains intact.
9. **Run targeted validation**: run prerequisite, onboarding, and any affected fixture/review gates with `CARGO_BUILD_JOBS=1` where appropriate.
10. **Do one cleanup PR per risk class**: keep archival moves separate from fixture/security/script changes so regressions are easier to isolate.

## Acceptance criteria mapping

- **No build behavior changes**: this audit document does not move, delete, or modify build inputs.
- **No verification behavior changes**: fixture and proof material is only recommended for review, not changed.
- **No CI regressions**: no CI configuration changed.
- **Easier to understand**: recommendations reduce root clutter and document source/archive/artifact boundaries.
- **Historical work preserved**: default recommendation is archival, not deletion.
- **First-time contributor experience improved**: root, docs, script, and fixture entry points become clearer.
- **Long-term public maintenance**: release artifacts, generated outputs, and secrets policy become explicit.
