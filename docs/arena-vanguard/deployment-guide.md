# Arena Vanguard Deployment Guide

Local proof generation:

```bash
cargo run -p everarcade-cli -- run-arena-local
cargo run -p everarcade-cli -- replay-world
```

The generated package and evidence live under `runtime/games/arena-vanguard/`:

- `package/world.toml`
- `package/genesis.json`
- `package/contract-package.json`
- `journal.json`
- `state.json`
- `receipts.json`
- `proofs/proof-bundle.json`
- `projection/index.html`

For live Evernode validation, deploy the package directory, start the runtime, submit the same serialized inputs, and archive the proof bundle after replay succeeds.
