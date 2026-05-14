# Linux VM Operator Quickstart

## Runtime Spine

EverArcade spine:

`package -> deterministic execution -> receipt -> replay verification -> checkpoint -> anchor intent`

- `execution-core` = deterministic truth.
- `everarcade-host` = operator/runtime layer that drives folder state and CLI lifecycle.

## Commands

1. Build: `cargo build --workspace`
2. Test: `cargo test --workspace`
3. Initialize state:
   - `cargo run -p everarcade-host -- init --state .everarcade`
4. Regenerate deterministic fixture:
   - `TMP_FIXTURE="$(mktemp)"
   - trap 'rm -f "$TMP_FIXTURE"' EXIT
   - cargo run -p everarcade-host -- generate-fixture --output "$TMP_FIXTURE"`
5. Execute package:
   - `cargo run -p everarcade-host -- run --package "$TMP_FIXTURE" --state .everarcade`
6. Verify replay/receipt/checkpoint/anchor continuity:
   - `cargo run -p everarcade-host -- verify --state .everarcade`
7. Inspect operator state:
   - `cargo run -p everarcade-host -- status --state .everarcade`
8. Print anchor intent path:
   - `cargo run -p everarcade-host -- anchor-intent --state .everarcade`

## Storage and Derived Data

Use `status --storage` to inspect current receipt/checkpoint/anchor counts and total bytes:

```bash
cargo run -p everarcade-host -- status --state .everarcade --storage
```

Derived data (indexes/caches/reports) can be rebuilt; canonical receipts/checkpoints/anchors remain protocol truth.

## Stress Flow

Run repeated deterministic execution and verification locally:

```bash
bash scripts/linux_vm_stress.sh
```
