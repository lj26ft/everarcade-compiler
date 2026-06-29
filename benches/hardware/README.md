# Continuum Boundary Benchmark Phase II: Hardware Harness

Phase II is a hardware torture campaign for discovering where the Continuum bends or breaks on a desktop CPU, 24 GB AMD GPU, and local NVMe storage. These harness definitions are intentionally measurement-first: they record capacity, bottlenecks, and failure modes rather than optimizing for the host.

## Local artifact policy

Benchmark runs MUST write raw outputs outside the committed tree at:

```text
.everarcade-continuum-phase-ii-review/artifacts/
```

Only curated markdown summaries in `reports/` are committed. Raw traces, receipts, checkpoints, snapshots, archives, flamegraphs, profiler captures, GPU dumps, and generated worlds are local reproduction artifacts and must not be added to pull requests.

## Hardware benchmark matrix

| Category | Scale points | Primary measurements | Break signal |
| --- | --- | --- | --- |
| CPU saturation | 1, 10, 100, 500, 1000 worlds | ticks/sec, worlds/sec, replay/sec, CPU utilization, context switches, scheduler latency | scheduler contention or missed tick budget |
| Memory saturation | 100 MB, 1 GB, 5 GB, 10 GB, 20 GB+ | memory/world, memory/receipt, memory/checkpoint, memory/aeon, paging onset | paging, OOM, replay degradation |
| Disk I/O saturation | journal, checkpoint, archive streams | writes/sec, reads/sec, snapshot time, restore time | persistence dominates tick or replay cost |
| Receipt saturation | 1k through 1b synthetic receipts | append cost, proof generation, proof verification, storage growth | Receipt MMR dominates interval cost |
| Checkpoint saturation | 1, 10, 100, 1000, 10000 tick epochs | checkpoint cost, size, verification, restore | epoch interval crosses latency/storage boundary |
| Replay saturation | genesis, checkpoint, epoch, aeon replay | replay cost per interval, checkpoint speedup | replay exceeds practical restore SLO |
| History saturation | 1, 5, 10, 25, 50, 100 simulated years | history size, checkpoint count, archive size, replay time, restore time | maximum practical aeon reached |
| Multi-world saturation | 1, 10, 100, 500, 1000, 5000 worlds | memory, storage, scheduler overhead, aggregate ticks/sec | one-machine carrying capacity exceeded |

## Required report outputs

Each completed run should update the corresponding report template:

- `reports/hardware_capacity_report.md`
- `reports/bottleneck_report.md`
- `reports/replay_interval_report.md`
