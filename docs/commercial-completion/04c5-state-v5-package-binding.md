# Task 4C5 — State V5 package binding

Status: complete.

The compiler binds these authoritative Runtime IR fields into canonical `world.evr` identity:

`state_policy_id`, `treasury_state_schema_id`, `treasury_commitment_policy_id`, `treasury_segmentation_policy_id`, `treasury_checkpoint_schema_id`, `treasury_migration_policy_id`, `initial_treasury_state`, `treasury_activation_policy`, `supported_treasury_action_inventory`, and `supported_treasury_receipt_schema_ids`.

The only accepted V5 tuple is State V5 / Treasury v3 / commitment v3 / segments v2 / recoverable checkpoint v2 / migration v1-to-v3 v2. Unknown and mismatched tuples fail closed. V2, V3, and V4 branches retain their historical validation and emitted fields.

`npm run test:state-v5-package` proves deterministic rebuild, direct package parsing, six policy-field mutation rejections, and genesis identity binding. The pinned Runtime IR hash is `sha256:1360d9ac1019f34ea7e095cd6c63c7030c356f2aac261a13ce0c02df6e296575`; the package hash is `sha256:e11c0f860b0997d026ba6d9171b07d0c60f2ed73745d08bc723bed52ac763e88`.
