# Runtime Boundaries

- `execution-core` owns deterministic parsing, planning, execution, receipt verification, and state-diff derivation from provided deterministic inputs.
- Host runtime owns filesystem, networking, process control, clocks/environment binding, and adapter conversion to core records.
- CLI owns argument parsing, UX/help/stdout formatting, and user-facing command compatibility.
- Runtime filesystem responsibilities are in host/CLI/scripts layers only.
- WASM boundary responsibilities: host performs memory bridge allocation/copy and exports/import binding; core defines canonical payload formats.
- ABI ownership: `execution-core` defines canonical ABI schema and validation semantics.
- State diff ownership: `execution-core` computes diff logic; host persists and transports it.
- Receipt ownership: `execution-core` computes/verifies deterministic receipt contents; host stores, distributes, and indexes receipts.

## Hard rule

`execution-core` may accept bytes, records, configs, manifests, and deterministic inputs. `execution-core` must not decide where those inputs come from.

## Forbidden in pure core

- Filesystem path resolution and file IO.
- Environment-variable decisions and wall-clock reads.
- Randomness/process spawning/global mutable singleton dependence.
