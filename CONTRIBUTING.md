# Contributing to EverArcade

Thank you for helping make EverArcade understandable and usable by external developers.

## Support boundary

EverArcade v0.1 is an open-source candidate focused on local deterministic runtime proofs. Do not describe local PASS reports as production, public-testnet, or commercial readiness.

## How to build and run the onboarding path

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
```

This creates a temporary Arena project, builds it through the Creator SDK, packages it, runs a playable local session, and verifies replay evidence.

## How to test your changes

Run targeted checks that match your change. For developer-experience or documentation work, run:

```bash
git diff --check
bash scripts/validate_open_source_readiness.sh
bash scripts/certify_developer_experience.sh
```

For runtime or Creator SDK changes, also run the relevant targeted script, for example:

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_playable_local_game.sh
```

Avoid broad workspace rebuilds unless a maintainer explicitly asks for them. Prefer targeted crate tests and use `CARGO_BUILD_JOBS=1` for large validation runs.

## Coding standards

- Keep scripts deterministic, non-interactive, and safe to run from the repository root.
- Make generated report paths explicit.
- Label scaffold, prototype, and production-candidate areas honestly.
- Do not add try/catch blocks around imports.
- Do not commit generated dependency trees, runtime roots, or local build outputs unless they are explicitly approved fixtures.
- Keep documentation claims aligned with `README.md`, `docs/repository/repository-map.md`, and `docs/runtime-platform/proof-chain.md`.

## Submitting changes

1. Create a focused branch.
2. Update or add documentation for behavior changes.
3. Run targeted validation scripts.
4. Ensure `git diff --check` passes.
5. Submit a pull request with:
   - summary of changed files;
   - commands run and results;
   - any remaining limitations or conditional readiness notes.

## Security-sensitive changes

Do not include secrets, credentials, wallet keys, or private operator configuration. If you find a vulnerability, follow `SECURITY.md` rather than opening a public exploit report.
