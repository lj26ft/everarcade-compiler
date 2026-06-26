# Repository Policy: Source, Artifacts, Releases, and Reproducibility

This repository preserves the engineering record required to reproduce and review EverArcade's local deterministic proof paths. Cleanup should reduce contributor confusion without erasing launch history or weakening reproducibility.

## What belongs in git

Git may contain:

- source code;
- specifications and trust-model documents;
- fixtures used by deterministic tests and review gates;
- tooling and scripts;
- documentation;
- reproducibility inputs such as lockfiles, pinned toolchain files, expected-output fixtures, and offline dependency bundles;
- historical review evidence that documents launch decisions and trust evolution.

## What belongs in GitHub Releases

GitHub Releases should contain generated or packaged outputs, including:

- generated release bundles;
- packaged runtime distributions;
- generated world artifacts such as release `world.evr` bundles;
- generated deployment/runtime archives intended for consumers rather than source review.

## Intentional exceptions

Some generated-looking files remain in git because they are reproducibility inputs or review fixtures rather than convenience build outputs.

- `dist/vendor.tar.gz` and its checksum are committed so contributors can restore Cargo dependencies offline after clone.
- Fixture keys under `fixtures/` and selected example fixtures are public test keys used to make deterministic local attestations reproducible.
- RC1, RC2, review bundles, and `TRUST_ROOT.md` are retained as launch-history and trust-model evidence.
- Expected-output fixtures may be committed when they are required for deterministic verification.

## What should not be committed

Do not commit local build outputs, runtime roots, generated reports, `vendor/`, `node_modules/`, private credentials, wallet keys, or operator secrets unless a maintainer explicitly classifies them as fixtures or reproducibility inputs.

## Cleanup philosophy

- Prefer moving or documenting over deleting.
- Archive uncertain historical material instead of removing it.
- Do not reorganize fixtures as part of incidental cleanup.
- Do not rename scripts until usage is documented and understood.
- Do not change trust boundaries, verification behavior, or determinism as part of documentation cleanup.
