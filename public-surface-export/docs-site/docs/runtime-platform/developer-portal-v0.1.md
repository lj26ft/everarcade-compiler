# Developer Portal v0.1

## Purpose

Developer Portal v0.1 is the first unified builder interface for EverArcade. It
turns existing runtime, deployment, settlement, federation, renderer, GPU, and
civilization surfaces into a replayable developer workflow. The portal is
non-authoritative: it creates records, manifests, views, and deployment intents,
but it does not mutate runtime state directly, bypass settlement, or bypass
protocol authority.

The operating path is:

```text
Developer -> Developer Portal -> EverArcade Runtime -> Deployment -> Civilization Runtime
```

## Layout

The portal scaffold lives in `developer-portal/`:

- `dashboard/` for the overview and root index.
- `projects/` for project registry records.
- `games/` for game creation and manifest records.
- `deployments/` for local, lease, and federation deployment views.
- `assets/` for asset, inventory, vault, and marketplace asset views.
- `wallets/` for authority, XRPL, Xaman, and settlement account views.
- `civilizations/` for civilization runtime views.
- `marketplace/` for listings, sales, royalties, revenue, and settlement events.
- `gpu/` for GPU marketplace capacity, assignment, artifact, and settlement intent views.
- `analytics/` for derived operational metrics.
- `onboarding/` for the shortest path to first deployment.
- `records/` for deterministic roots.

## Project Registry

The project registry records project ID, developer ID, project name, project
type, version, and status. Lifecycle actions are create, update, publish, and
archive. The Project Registry Root is derived from a canonical transcript so
registry state can be audited without treating the portal as an authority.

## Game Creation

The game creation flow represents create project, create game, generate runtime
manifest, and generate deployment manifest. It emits a Game Creation Root that
links the project registry to runtime and deployment surfaces.

## Deployments

The deployment dashboard represents local deployment, lease deployment, and
federation deployment. Each record includes status, health, checkpoint, and
replay identifiers. The Deployment Root is derived from those records and the
Game Creation Root.

## Assets

Asset management represents assets, inventories, vault assets, and marketplace
assets. The portal observes ownership and custody boundaries through renderer,
runtime, settlement, and marketplace surfaces. The Asset Registry Root is the
replayable audit anchor for these views.

## Wallets

The wallet dashboard represents authorities, XRPL wallets, Xaman connections,
and settlement accounts. It explicitly has no private key custody. The Wallet
Root proves the portal is observing and coordinating wallet surfaces without
holding keys.

## Civilizations

The civilization dashboard represents civilizations, worlds, regions,
governance, and economies. The Civilization Root links these records to the
Civilization Runtime while preserving runtime authority.

## Marketplace

The marketplace dashboard represents listings, sales, royalties, creator
revenue, and settlement events. The Marketplace Root records observed commerce
activity and settlement events; live payments and billing remain out of scope.

## GPU Marketplace

The GPU dashboard represents GPU providers, capacity, assignments, artifacts,
and settlement intents. The GPU Marketplace Root is an operating view over the
existing GPU Marketplace and GPU Runtime. The portal does not assign authority
outside the marketplace protocol and does not settle payments directly.

## Analytics

The analytics layer represents deployments, players, settlements, marketplace
activity, GPU activity, and civilization activity. Analytics are derived
observations only and are not authoritative state.

## Onboarding

Developer onboarding is the shortest path from a new developer to first
deployment:

1. Create project.
2. Build with the existing CLI/runtime tooling.
3. Validate records and manifests.
4. Deploy locally, by lease, or by federation.
5. Monitor health, checkpoints, replay, and activity.

The Onboarding Root anchors this path for replay and audit.

## PASS Criteria

Developer Portal v0.1 passes when validation and certification report PASS for:
projects, game creation/games, deployments, assets, wallets, civilizations,
marketplace, GPU, analytics, and onboarding. Reports must include deterministic
roots for each domain and an overall Developer Portal PASS.

## FAIL Criteria

The portal fails if any required domain is missing, any root is not a valid
64-character SHA-256 digest, private key custody is introduced, settlement or
authority bypass appears, or validation/certification scripts fail.

## Relationship To GPU Marketplace

The portal consumes GPU Marketplace records as a dashboard and onboarding
surface. GPU providers, capacity, assignments, artifacts, and settlement intents
remain governed by the GPU Marketplace. The portal exposes them to builders but
is not a replacement scheduler, verifier, or payment authority.

## Relationship To Future Public Testnet

Developer Portal v0.1 prepares the platform for Public Testnet v0.1 by giving
builders a unified workflow before public hosting, production authentication,
OAuth, multi-tenant billing, live payments, and real accounts are introduced.
The next milestone can move from infrastructure complete to ecosystem ready
because developers can build, deploy, monetize, and operate games without deep
protocol knowledge.
