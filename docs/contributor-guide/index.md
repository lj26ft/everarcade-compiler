# Contributor Guide

Start with the public docs and roadmap while the repository completes its security, license, and release audit. Repository-specific contribution instructions will be published with the open-source release.

## Topics

- Repository structure: coming with the open-source repository release.
- Build instructions: [offline build policy](../build/offline-build-policy.md), [CLI quickstart](../CLI_QUICKSTART.md)
- Testing: prefer targeted crate or script validation; do not run full workspace tests unless required.
- Pull request workflow: keep documentation updates with behavior changes and cite maturity changes in `MATURITY.md`.
- Coding standards: follow local conventions, keep imports direct, and avoid changing protocol or runtime semantics during documentation-only work.

## Documentation contributions

Use [`docs/DOCUMENTATION_POLICY.md`](../DOCUMENTATION_POLICY.md) to decide whether new material belongs in canonical docs, subsystem README files, or `archive/`.
