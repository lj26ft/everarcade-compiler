# Task 4D.1D — State V7 package binding

The canonical request and Runtime IR now bind `treasury_semantic_root_policy_id` and `treasury_materialization_plan_policy_id` in addition to the State V6 semantic fields. State V7 requires Treasury v5, commitment v5, semantic roots v1, the pinned Task 4D.0 policy commitments, rejection evidence v2, active/archive policy v1, and materialization plan v1.

`npm run test:state-v7-package` proves deterministic bytes, Runtime IR/package identity binding, Authority package verification, and rejection of a mutated semantic-root policy. Historical State V5 and State V6 package tests remain valid.
