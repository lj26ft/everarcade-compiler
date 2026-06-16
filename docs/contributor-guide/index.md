# Contributor Guide

Start with the root [`CONTRIBUTING.md`](https://github.com/everarcade/everarcade-compiler/blob/main/CONTRIBUTING.md), then use this page for repository-specific orientation.

## Topics

- Repository structure: [`REPOSITORY_MAP.md`](https://github.com/everarcade/everarcade-compiler/blob/main/REPOSITORY_MAP.md)
- Build instructions: [offline build policy](../build/offline-build-policy.md), [CLI quickstart](../CLI_QUICKSTART.md)
- Testing: prefer targeted crate or script validation; do not run full workspace tests unless required.
- Pull request workflow: keep documentation updates with behavior changes and cite maturity changes in `MATURITY.md`.
- Coding standards: follow local conventions, keep imports direct, and avoid changing protocol or runtime semantics during documentation-only work.

## Documentation contributions

Use [`docs/DOCUMENTATION_POLICY.md`](../DOCUMENTATION_POLICY.md) to decide whether new material belongs in canonical docs, subsystem README files, or `archive/`.
