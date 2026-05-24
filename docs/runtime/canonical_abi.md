# Canonical ABI Semantics (Protocol Law)

- Encoding: UTF-8 JSON bytes from `serde_json::to_vec`.
- Request envelope fields and order: `version`, `payload`.
- Request payload fields and order: `contract_id`, `input`.
- Response envelope fields and order: `mutations`, `stdout`, `status`.
- Mutation entry shape: `[key, value_bytes]`.
- Determinism constraints:
  - no host entropy/time/env usage in guest
  - canonical ordering preserved by serializer declaration order
  - replay identity defined by byte-equivalent request/response envelopes.

Any ABI parsing or envelope decode failure is canonical `MalformedAbi`.
