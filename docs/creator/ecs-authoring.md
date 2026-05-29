# Deterministic ECS Authoring

The ECS editor creates deterministic entities, components, systems, and archetypes. Component schemas are canonicalized and system order must be stable.

Hidden ECS mutation and non-canonical system ordering are rejected because they would break replay equivalence.
