# Vercel Docs Deployment Readiness Report

Date: 2026-06-16

## Errors Found

- Docusaurus rejected the sidebar because `13-runtime-operations-manual` no longer exists as a document ID.
- Docusaurus v3 emitted a deprecation warning for the root-level `onBrokenMarkdownLinks` config key.
- Docusaurus emitted duplicate route warnings for `/docs/` and `/docs/architecture/` because README files produced the same routes as index documents.
- The docs build failed with unresolved `@mermaid-js/layout-elk` while compiling the Mermaid theme.
- Docusaurus found broken internal links from public pages and docs pages to stale numbered paths.

## Sidebar IDs Fixed

- Replaced `13-runtime-operations-manual` with `runtime-operations-manual` in `sidebars.js`.

## Duplicate IDs / Routes Fixed

- No duplicate document ID failure remained after the sidebar fix.
- Resolved duplicate `/docs/` and `/docs/architecture/` routes by assigning explicit slugs to `docs/README.md` and `docs/architecture/README.md`.

## Broken Links Fixed

- Updated root repository file references in getting-started and contributor docs to GitHub URLs.
- Updated architecture system-overview links from stale numbered paths to current canonical paths.
- Updated public page links from stale numbered docs paths to current Docusaurus routes.

## Docusaurus Config Changes

- Moved broken Markdown link handling from deprecated `siteConfig.onBrokenMarkdownLinks` to `siteConfig.markdown.hooks.onBrokenMarkdownLinks`.
- Added `@mermaid-js/layout-elk` to dependencies so Docusaurus Mermaid builds resolve under the current dependency tree.

## Vercel Configuration

Added root `vercel.json`:

```json
{
  "framework": null,
  "installCommand": "npm install",
  "buildCommand": "npm run docs:build",
  "outputDirectory": "build"
}
```

Intended Vercel settings remain:

- Framework Preset: Other
- Install Command: `npm install`
- Build Command: `npm run docs:build`
- Output Directory: `build`
- Root Directory: repository root

## Validation Commands Run

```bash
npm install
npm run docs:build
git diff --check
git status --short
find . -type f -size +500k | sort
```

## Final Results

### `npm install`

Result: passed.

Notes:

- npm reported `Unknown env config "http-proxy"`; this is an environment warning.
- npm audit reported 30 existing dependency vulnerabilities. No audit remediation was attempted because it is outside this documentation deployment readiness milestone.

### `npm run docs:build`

Result: passed.

Final build output included:

```text
[SUCCESS] Generated static files in "build".
[INFO] Use `npm run serve` command to test your build locally.
```

### `git diff --check`

Result: passed with no whitespace errors.

### `git status --short`

Result after validation and before staging:

```text
 M .gitignore
 M docs/README.md
 M docs/architecture/README.md
 M docs/architecture/index.md
 M docs/architecture/system-overview.md
 M docs/concepts/index.md
 M docs/contributor-guide/index.md
 M docs/getting-started/index.md
 M docusaurus.config.js
 M package.json
 M sidebars.js
 M src/pages/contributors.md
 M src/pages/developers.md
 M src/pages/operators.md
?? package-lock.json
?? runtime/gpu-marketplace/Cargo.lock
?? scripts/validate_docs_deployment.sh
?? vercel.json
```

`runtime/gpu-marketplace/Cargo.lock` was pre-existing untracked state and was not staged for this docs deployment change.

### `find . -type f -size +500k | sort`

Result: completed successfully.

Known generated/local large files were found under ignored `build/`, `node_modules/`, `.git/`, and existing dependency/runtime directories. None of the generated Docusaurus output, dependency folders, screenshots, videos, or large binaries were staged.

## Remaining Warnings

- `npm install` reports an environment warning for `http-proxy`.
- `npm install` reports audit findings in the dependency tree.
- The size scan lists local large files because it intentionally scans the full working tree, including ignored build and dependency directories.

## Known Non-Blockers

- Local ignored artifacts (`node_modules/`, `build/`, `.docusaurus/`) are produced by validation and are excluded by `.gitignore`.
- `runtime/gpu-marketplace/Cargo.lock` is unrelated untracked working tree state and is not part of this commit.
