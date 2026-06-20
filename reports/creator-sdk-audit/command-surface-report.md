# Creator SDK Command Surface Report

## Current Creator SDK commands

The current Node CLI advertises:

```text
everarcade <new|build|test|package|certify-world|verify-world-certificate|launch-local|execute-local|execute-template|execute-guest|play-local|play-local-multiplayer|play-network-local|play-federated-local|play-multi-lease-local|deploy|publish> [--project DIR]
```

In practice, the executable is normally invoked as:

```bash
node creator-sdk/cli/everarcade.mjs <command>
```

## Desired command surface

```bash
everarcade world init
everarcade world add-rustrig
everarcade world package
everarcade world verify
everarcade world project
everarcade world deploy
everarcade world replay
everarcade world restore
everarcade world migrate
```

## Comparison

| Desired command | Current equivalent | Status | Problem |
| --- | --- | --- | --- |
| `everarcade world init` | `new` | Missing alias | Current command does not use world lifecycle language. |
| `everarcade world add-rustrig` | None | Missing | RustRig selection is docs/manual only. |
| `everarcade world run` | `play-local`, `launch-local`, `execute-local`, `execute-template`, `execute-guest` | Duplicated/confusing | Too many run-like commands for a first run. |
| `everarcade world package` | `package` | Missing alias | Works but package type is unclear. |
| `everarcade world verify` | `test`, `certify-world`, `verify-world-certificate`, replay checks | Missing aggregate | Verification is fragmented. |
| `everarcade world project` | None in Creator SDK | Missing | Projection requires architecture/example discovery. |
| `everarcade world deploy` | `deploy` | Missing alias | Local deployment metadata exists, but command is generic. |
| `everarcade world replay` | Runtime proof commands indirectly | Missing | Replay is not creator-first. |
| `everarcade world restore` | Runtime/deployment scripts | Missing | Restore is operator/runtime-first. |
| `everarcade world migrate` | Deployment docs mention migration | Missing | Not exposed to creator workflow. |

## Confusing or too verbose operations

- `play-local`, `launch-local`, `execute-local`, `execute-template`, and `execute-guest` sound like alternate first-run paths.
- `certify-world` and `verify-world-certificate` sound more advanced than “verify my package”.
- `deploy` and `publish` are easy to confuse before the user knows deployment targets.
- Required invocation with `node creator-sdk/cli/everarcade.mjs` is too verbose for docs aimed at creators.

## Recommendation

Keep existing commands for compatibility, but add a creator-first umbrella:

```bash
everarcade world init --template frontier
everarcade world add-rustrig combat
everarcade world run
everarcade world package
everarcade world verify
everarcade world project
everarcade world deploy
```

Advanced operations should remain available behind explicit flags or subcommands after the first world succeeds.
