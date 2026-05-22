# Client Bridge
Client bridge lives in `sdk/client-bridge/` and protocol messages in `execution-core/src/client_protocol/`.
Rule: client submits commands only; runtime owns state.
APIs: `get_world_snapshot`, `get_entity_snapshot`, `submit_player_command`, `subscribe_simulation_events`, `load_asset_manifest`.
