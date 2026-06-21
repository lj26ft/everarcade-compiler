# Live Evernode world deployment v0.1

Operator flow:

```sh
everarcade release package
everarcade release verify
everarcade world init
everarcade world package
everarcade world verify
everarcade lease acquire
everarcade world deploy
everarcade lease health
everarcade world replay
everarcade world migrate
everarcade world recover
```

A live proof starts with a verified `world.evr`, then builds a lease-specific `runtime-bundle.zip` containing the HotPocket adapter entrypoint, canonical transport bridge metadata, the real EverArcade runtime binary, world manifest, genesis state, schemas, and verification metadata.

Lease identity is temporary compute placement: host account, tenant account, ports, instance public key, and runtime bundle target. World identity is sovereign: world id, manifest hash, genesis root, state root, replay root, receipt root, and continuity root. Migrating or recovering a lease must not change the world package hash or continuity root.

Deployment evidence is captured in `reports/live/` and lease reports in `reports/lease/`: lease acquisition, deployment, health, canonical mutation submission, runtime receipt, and local root verification. A valid receipt must parse as a transport receipt and include state root, replay root, receipt root, and continuity root. Local replay is successful only when it reproduces the live receipt roots.
