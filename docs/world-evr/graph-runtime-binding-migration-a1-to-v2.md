# Graph Runtime binding migration: A1 to Binding V2

Status: candidate migration profile, ready for authenticated governance adoption.

The predecessor remains `WORLD-EVR-V1-CONTRACT-01A1`, Adapter
`everarcade.world-graph-adapter.v1`, Binding
`everarcade.world-graph-binding.v1`, and its exact historical Graph Runtime
identity. This candidate does not rewrite or broaden A1.

The successor is Adapter `everarcade.world-graph-adapter.v2` version 2.0.0,
Binding `everarcade.world-graph-binding.v2` version 2.0.0, and exactly
`GRAPH-RUNTIME-VERTICAL-SLICE-RC1` at commit
`cbcc49166e3412b07a2e4d2fb7102b72a7fa7ade` and tree
`b6b030654ce9ebf74e1f7af1abcc0c3862dd3823`.

New packages select V2 explicitly with
`--graph-adapter everarcade.world-graph-adapter.v2`. Packages created without
that selector retain the historical V1 profile. Existing A1 packages and
running or recovered A1 worlds remain bound to Adapter 01 and its accepted
runtime. They are never reinterpreted as V2 merely because newer software is
installed.

A historical package may move to V2 only through a separately governed
package/runtime rebinding or migration decision that emits new binding and
release metadata. Portable-state transitions preserve the source package's
governed runtime binding; they do not silently change it. New V2 packages bind
the canonical operation identity
`everarcade.graph.current-head-reconstitution.v1`.

The compiler change is a metadata/profile selection addition. It does not
alter world compilation semantics. Governance adoption is deliberately
outside this implementation qualification and is expected in
`WORLD-EVR-V1-CONTRACT-01A3`.
