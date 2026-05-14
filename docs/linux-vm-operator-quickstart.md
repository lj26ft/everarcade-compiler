# Linux VM Operator Quickstart

1. Prerequisites: Rust stable + cargo on Linux VM.
2. Build: `cargo build --workspace`
3. Test: `cargo test --workspace`
4. Init: `cargo run -p everarcade-host -- init --state .everarcade`
5. Run fixture: `cargo run -p everarcade-host -- run --package everarcade-host/tests/fixtures/civilization_package.bin --state .everarcade`
6. Verify: `cargo run -p everarcade-host -- verify --state .everarcade`
7. Status: `cargo run -p everarcade-host -- status --state .everarcade`
8. Anchor intent: `cargo run -p everarcade-host -- anchor-intent --state .everarcade` prints latest local anchor intent artifact.
9. Dry-run vs live adapters: default is local deterministic dry-run; live adapters are optional feature gates.
10. execution-core does NOT perform networking, settlement submission, IPFS publication, federation consensus, or governance automation in the default operator hot path.
