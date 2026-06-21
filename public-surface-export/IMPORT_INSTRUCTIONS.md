# Import Instructions for `everarcade-frontend`

## Source paths in `everarcade-compiler`

| Export path | Original source path(s) | Intended destination in `everarcade-frontend` |
| --- | --- | --- |
| `docs-site/` | `docusaurus.config.js`, `vercel.json`, `docs/`, public launch/narrative root docs | `apps/docs-site/` or the frontend repo's docs app root |
| `vision-site/` | `docs/vision.md`, `docs/founding-worlds.md` | `apps/vision-site/content/` |
| `world-portal/` | `frontend/world-portal/`, `developer-portal/` | `apps/world-portal/` with records under `fixtures/developer-portal/` |
| `registry-fixtures/` | `public-testnet/`, `registry/` | `fixtures/registry/` and/or `apps/*/fixtures/` |
| `deployment-notes/` | public launch checklist, testnet page, public testnet runbooks | `docs/deployment/` or project onboarding notes |
| `x-positioning/` | `public-social/` | `content/social/` |

## Build commands

Run these after copying into `everarcade-frontend` and adapting package manager/workspace paths:

```bash
# Docs site (Docusaurus)
npm install
npm run build

# World portal (Vite)
cd apps/world-portal
npm install
npm run build
```

If the frontend repo uses a monorepo package manager, translate the commands to the repo convention (for example `pnpm install`, `pnpm --filter docs-site build`, or `pnpm --filter world-portal build`).

## Domain mapping

- `docs.everarcade.games` should serve the imported docs site.
- `vision.everarcade.games` should serve the imported vision site/pages.
- The world portal should be mapped by the frontend repo's product routing decision, for example `portal.everarcade.games` or an app route under the main web property.

## Vercel project notes

- Treat `vercel.json` in `docs-site/` as historical source config to merge into the frontend repo's active Vercel project configuration.
- Create separate Vercel projects only if the frontend repo chooses independent deployments for docs, vision, and portal.
- Keep production domains owned by `everarcade-frontend`; `everarcade-compiler` should not deploy public website projects.
- Verify rewrites/headers after import because package-root paths will likely change.

## Compiler handoff boundary

After import, `everarcade-compiler` should link outward to `everarcade-frontend`, `https://docs.everarcade.games`, and `https://vision.everarcade.games` instead of owning public presentation content.
