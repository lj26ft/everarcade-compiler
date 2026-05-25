# Public API Manifest

| Surface | Type |
| --- | --- |
| public structs | PublicApiSurfaceHash, ProtocolSurfaceHash |
| public enums | ExecutionStatus (runtime-facing) |
| public traits | none (currently) |
| public functions | runtime_validation_root, public_api_surface_hash, protocol_surface_hash |
| public modules | api::{runtime, execution, continuity, restoration, validation} |
| protocol-law surfaces | RuntimeValidationRoot, ExecutionReceipt, WorldSnapshot |
| internal-only surfaces | federation sync internals, security plumbing helpers |
