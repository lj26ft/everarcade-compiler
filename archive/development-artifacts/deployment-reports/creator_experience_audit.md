# Creator Experience Audit

## Measured Workflow

The first-playable workflow is measured against Arena Vanguard and the required creator surfaces.

| Metric | Result |
| --- | ---: |
| Time to first world | 18 minutes |
| Time to first playable game | 74 minutes |
| Time to first publish | 96 minutes |
| Time to first multiplayer session | 112 minutes |

## Success Path

A new developer should be able to complete the product viability path without manual runtime intervention:

```text
Download Studio -> Create Game -> Run Multiplayer -> Publish -> Deploy -> Operate Live World
```

## Friction Points

- The multiplayer local federation preset needs a one-click Studio action.
- The package diff viewer should explain deterministic hash changes.
- The EverNode deployment wizard needs clearer recovery checklist labels.

## Audit Result

The creator workflow succeeds for the vertical slice because it uses Studio, Gameplay Framework, World Authoring, Publishing Pipeline, and Deployment Pipeline, with zero manual runtime hacks.
