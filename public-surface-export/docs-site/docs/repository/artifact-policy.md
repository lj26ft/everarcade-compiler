# Artifact Cleanup Policy

This policy defines what belongs in git and what should remain generated locally.

## Commit these

- Source code, templates, scripts, and hand-written documentation.
- Small deterministic fixtures needed by tests.
- Canonical audit reports requested by milestones when they summarize repository status.
- Governance files: `LICENSE`, `CONTRIBUTING.md`, `SECURITY.md`, and `CODE_OF_CONDUCT.md`.

## Do not commit these by default

- `target/`, `dist/`, `build/`, cache directories, temporary runtime roots, and local package outputs.
- `node_modules/` and package-manager install trees.
- Generated local sessions, replay roots, receipt streams, journals, transcripts, checkpoints, and runtime evidence unless explicitly curated as a small fixture.
- Secrets, keys, tokens, wallet credentials, local environment files, and production operator configuration.
- Large dependency archives unless approved as release artifacts with checksums.

## Generated reports

Reports under `reports/` may be committed only when they are milestone summaries, canonical audits, or requested certification outputs. Routine local validation logs should be regenerated instead of committed.

## Generated packages

Creator SDK runtime packages are generated under project `dist/` directories and should not be committed unless a maintainer explicitly promotes one to a small fixture. The canonical package proof is the script and report, not a stale package copy.

## Validation outputs

Validation scripts should write predictable reports in `reports/`. Runtime roots used during validation should be temporary and cleaned up by the script.

## Runtime evidence

Runtime evidence is useful for debugging, but it can be misleading when copied across dates or environments. Commit only curated evidence with clear labels, provenance, and a matching validation report.
