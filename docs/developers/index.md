# Developer Onboarding

Developers create worlds and validate local execution.

## Topics

- Creating worlds: [Creator SDK quick start](../creator-sdk/quick-start.md), [build your first game](../sdk/build-your-first-game.md)
- World packages: [world package v0.1](../world-package/world-package-v0.1.md), [canonical package format](../canonical-package-format.md)
- Runtime packages: [deployable runtime bundle](../world-package/deployable-runtime-bundle-v0.1.md), [runtime package bridge](../creator-sdk/runtime-package-bridge.md)
- World contracts: [contract development](../architecture/sdk/contract-development.md), [WASM game ABI](../wasm-game-abi-v0.1.md)
- RustRigs: [RustRigs documentation](../rustrigs/index.md)
- Creator SDK: [SDK guide](../developer/sdk-guide.md), [Creator SDK README](../creator-sdk/README.md)
- Local workflow: [30-minute journey](../onboarding/30-minute-developer-journey.md), [local federation guide](../developer/local-federation-guide.md)

## Recommended local validation

Run targeted onboarding validation rather than full workspace tests unless explicitly needed:

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
```
