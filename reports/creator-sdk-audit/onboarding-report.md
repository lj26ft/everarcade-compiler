# Creator SDK Onboarding Report

## Starting point

Assumption: the developer runs `git clone everarcade` and opens the repository with no EverArcade context.

## Current onboarding path

1. Open `README.md`.
2. Install prerequisites: Node.js 18+, Rust/Cargo, and network access.
3. Run either the onboarding validation script or the manual local flow.
4. Create an `arena` project using the Node CLI.
5. Build the project.
6. Test the manifest.
7. Package the runtime bundle.
8. Run `play-local` with a runtime root.
9. Confirm `Playable Local Game: PASS`.

## First-time developer answers

| Question | Can answer without architecture book? | Notes | Score |
| --- | --- | --- | --- |
| What do I build? | Partially | “Create a local game from the Creator SDK” is clear; “World” vs “Game” is mixed. | F2 |
| Where do I start? | Yes if README is trusted | README quick start is effective. | F1 |
| Which template do I choose? | Not confidently | `arena` works, but no template picker explains choices. | F3 |
| Which RustRigs do I need? | No | No CLI mapping from gameplay goal to RustRig. | F4 |
| How do I run it? | Yes, but verbose | `play-local` path works. | F2 |
| How do I package it? | Partially | `package` works, output meaning unclear. | F3 |
| How do I verify it? | No single answer | Test, certificate verification, and replay verification are separate. | F4 |
| How do I deploy it? | Partially | Local deployment record exists; live deployment maturity is unclear. | F3 |

## Onboarding failures

- The first successful path is command-heavy and exposes implementation paths.
- Template choice is not self-service.
- World lifecycle words are inconsistent: game, world, package, runtime package, deployment, publication.
- RustRig discovery is separate from project creation.
- Projection is not part of the first-run workflow.
- Verification is not a single plain-English step.

## Time To First World

Current **Time To First World** is likely acceptable for maintainers but fragile for first-time developers. The main delays are not compilation; they are vocabulary, command discovery, and confidence that the generated files are the correct artifacts.
