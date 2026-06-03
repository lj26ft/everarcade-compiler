# EverArcade Runtime Layout for Evernode

EverArcade Runtime Platform v0.1 stores all authoritative world data outside the source tree so release artifacts can run on a Linux host or Evernode lease.

## Lease filesystem

Default persistent root:

```text
/var/lib/everarcade/
├── packages/
└── worlds/
    └── <world-id>/
        ├── state/
        ├── journals/
        ├── checkpoints/
        ├── receipts/
        ├── backups/
        └── runtime.json
```

The layout is versioned by the runtime persistence envelope. Operators may relocate the root with runtime configuration, but package, world, journal, checkpoint, receipt, and backup paths remain stable relative to that root.

## Persistence paths

- `worlds/<world-id>/state/` stores canonical state snapshots owned by the runtime.
- `worlds/<world-id>/journals/journal.jsonl` is the append-only, hash-chained replay source.
- `worlds/<world-id>/checkpoints/` stores versioned checkpoint manifests plus state snapshots.
- `worlds/<world-id>/receipts/` stores versioned execution receipts.
- `worlds/<world-id>/runtime.json` stores runtime health and operator-visible status.

## Package paths

Packages are staged under `packages/` and must include a `manifest.json`, WASM payload, package hash metadata, runtime compatibility, and signature metadata. Startup fails during package validation before any world execution if package validation fails.

## Backup paths

Backups are stored under `worlds/<world-id>/backups/<backup-id>/` with:

- `manifest.json` containing backup metadata.
- `checkpoint.json` containing the exported checkpoint.

Backups are verified by comparing backup metadata against checkpoint integrity hashes.

## Recovery paths

Automatic recovery loads the latest checkpoint from `checkpoints/`, replays `journals/journal.jsonl`, recomputes roots, verifies continuity, and resumes the runtime. Journal data remains authoritative after the checkpoint cursor.

## Upgrade paths

The upgrade flow is:

1. Create a rollback backup.
2. Validate package version and runtime compatibility.
3. Validate migration and replay equivalence.
4. Promote the upgraded package.
5. Roll back to the backup on failure.

Release artifacts must not assume access to the source tree.
