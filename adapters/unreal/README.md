# Unreal Adapter Design

Unreal support is architecture-only in SDK v1. No implementation is required initially.

## Proposed modules

- **World Client**: Unreal subsystem responsible for connection lifecycle.
- **Input Submitter**: Blueprint/C++ API that submits gameplay intent to the World Contract API.
- **Projection Reader**: Component or subsystem that maps projections onto actors, components, and UI.
- **Verification Client**: Optional C++ module for package, receipt, and projection proof verification.

## Boundary

Unreal keeps rendering, Blueprints, assets, animation, UI, and presentation physics. EverArcade owns authoritative state transition, RustRigs, World Contracts, World Packages, receipts, and projection commitments.
