# EverNode Deployment Trial

## Bridge Artifacts
- deployment manifest: `deployment/evernode/deployment_manifest.toml`
- runtime manifest: `deployment/evernode/runtime_manifest.toml`
- package manifest: `deployment/evernode/package_manifest.toml`
- world manifest: `deployment/evernode/world_manifest.toml`

## Operations
- deploy: deterministic local simulation
- start: deterministic local simulation
- stop: deterministic local simulation
- restart: deterministic local simulation
- recover: deterministic local simulation
- verify: deterministic local simulation

## Local EverNode Simulation
- single node: validated
- multi node: validated
- restart: validated
- failure: validated
- recovery: validated

## Authority Boundary
The runtime produces deployment records and manifests. Live EverNode submission is performed by an external operator bridge, not by runtime authority.
