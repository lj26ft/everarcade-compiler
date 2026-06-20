# EverArcade v0.1 Open Source Release Candidate

Status: PASS candidate for public repository review.

This release candidate freezes the public v0.1 onboarding story around the Creator SDK and local runtime proof path. It does not introduce new platform architecture.

## Primary workflow

Use the Creator SDK CLI from a source checkout:

```bash
node creator-sdk/cli/everarcade.mjs world templates
node creator-sdk/cli/everarcade.mjs world rustrigs
node creator-sdk/cli/everarcade.mjs world init --template frontier --name frontier-world --dir "$PROJECT"
CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs world run --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world package --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world verify --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world deploy --project "$PROJECT"
```

The supported public nouns are World, World Package, World Contract, RustRig, Projection, and Deployment.

## Repository inventory summary

- ACTIVE: `creator-sdk/cli`, `creator-sdk/templates`, `crates/canonicalizer-kernel`, `crates/rustrigs/*`, `runtime/everarcade-runtime`, `runtime/hotpocket-adapter`, docs required for first-world onboarding.
- SUPPORTED: `examples/reference-certified-world-v1`, `examples/world-creation-flow/frontier-validation`, `runtime/games/arena-vanguard`, `docs/first-world.md`, `docs/creator-sdk/README.md`, `docs/world-template-library-v1.md`, `docs/rustrigs/rustrig-suite-v1.md`.
- EXPERIMENTAL: renderer, history, federation, GPU marketplace, XRPL/Xahau, public-testnet, commercial-revenue, and HotPocket proof harnesses unless marked otherwise by maturity docs.
- OBSOLETE: duplicate pre-v1 reference package directories should be archived after public release cutover.
- UNKNOWN: generated proof remnants and root-level legacy helper scripts should remain non-primary until audited per release report.

## Release gates

- README first-world flow is present.
- Required release docs are present.
- Security refresh found no committed production secrets, credentials, or private keys.
- Generated artifacts are either ignored, documented, or noted for cleanup.
- Independent verification story is documented in `docs/verification-story-v1.md`.

See `reports/release-candidate-audit/` for the detailed audit packet.
