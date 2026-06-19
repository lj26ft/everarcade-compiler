# World Package Certification vNext

World Package Certification maps local formal proof artifacts into a package-level certificate without changing the v0.1 runtime architecture, runtime authority, or canonicalizer.

The local certification flow is:

```text
world.evr
↓
certified kernel(s)
↓
signed certificate
↓
independent verification
↓
deploy
```

## Certification artifacts

Running `certify-world` writes package-level certification files under `dist/certification/`:

- `formal-proof-registry.json` records the proof registry for the package, including manifest, world metadata, and kernel artifact digests.
- `world-package-certificate.json` is the signed package certificate. Its signature is a deterministic `sha256:` digest over the certificate payload.
- `independent-proof-recheck.json` records the independent re-check results for the certificate signature and all referenced artifact digests.
- `package-certification-artifacts.json` is the deploy-facing artifact map for the full `world.evr → deploy` chain.

The `deploy` command does not alter runtime authority. When a signed certificate exists, deploy records the certificate path, signature, and independent re-check status in `dist/deployment.json`.

## Commands

```bash
node creator-sdk/cli/everarcade.mjs certify-world --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs verify-world-certificate --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs deploy --project "$PROJECT"
```

Expected local results:

```text
World Package Certification: PASS
Independent Proof Re-check: PASS
Deploy: PASS (<project-name>)
```

## Scope boundaries

This milestone only maps proof artifacts into certification records. It intentionally does not change:

- v0.1 architecture;
- runtime authority;
- canonicalizer behavior;
- production readiness status.
