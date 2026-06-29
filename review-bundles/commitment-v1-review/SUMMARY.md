# Summary

## Included Scope

- Commitment Architecture V1
- Receipt MMR V1
- Checkpoint V1
- Proof Format V1
- Reference implementation
- Conformance vectors
- Continuum benchmark suite

## Source Provenance

- Created from commit: `94f79f39b15b77342f5eacfecc6a360cc96d163d`
- Recent merged commits:
- 94f79f3 Merge pull request #467 from lj26ft/codex/create-commitment-architecture-v1-spec
- 6f74ed2 Merge pull request #466 from lj26ft/codex/improve-repository-discoverability-and-onboarding
- f55cd1b Merge pull request #465 from lj26ft/codex/add-verification.md-for-review-attribution
- b02d58e Merge pull request #463 from lj26ft/codex/conduct-commercial-boundary-audit-for-repo
- ea1f1da Merge pull request #464 from lj26ft/codex/remove-commercial-strategy-from-open-source-repo

## CI Status

- Status: not queried by this bundle-generation step.
- Reviewer action: validate the source commit in the repository hosting system and rerun the verification steps in `VERIFY_BUNDLE.md`.

## Benchmark Status

- Benchmark report inventory is present under `benchmarks/`.
- Reports that were not available at the repository root during packaging are marked in-place as missing evidence and should be regenerated or supplied before relying on benchmark conclusions.


## Phase II Continuum Benchmarking

Phase II Continuum benchmark reports are included under `reports/phase-ii/` for independent inspection with the V1 commitment materials. The reports summarize local probes for:

- real hardware limits
- CPU saturation
- memory saturation
- disk I/O saturation
- replay interval cost
- determinism repeatability
- GPU exploration
- catastrophe/adversarial behavior

Raw benchmark artifacts remain local under `.everarcade-continuum-phase-ii-review/artifacts/` and are intentionally excluded from committed review-bundle contents and generated archives.
