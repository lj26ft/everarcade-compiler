# Binding migration: A1 to V2

FROM: `WORLD-EVR-V1-CONTRACT-01A1`, Adapter 01, Binding V1, and the exact
historical Graph Runtime commit `a561207bcc3b953b036102c805620ab97d50c344`.

TO: Adapter 02 version 2.0.0, Binding V2 version 2.0.0, and exact frozen Graph
Runtime RC1 commit `cbcc49166e3412b07a2e4d2fb7102b72a7fa7ade`, tree
`b6b030654ce9ebf74e1f7af1abcc0c3862dd3823`.

Existing historical packages, running worlds, and recovered worlds remain
bound to A1 unless a separate governed rebinding/migration emits new package
and release metadata. Portable reconstitution preserves the governing binding
and cannot silently upgrade it. Newly compiled V2 packages explicitly select
Adapter 02 and record Binding V2 and RC1. A governance acceptance decision is
still required before this candidate may be used by PIPELINE-02.
