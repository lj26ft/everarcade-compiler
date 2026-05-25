#[test]
fn parses_dag_from_bytes() {
    let json =
        br#"{"nodes":[{"id":"a","action":"noop","deps":[],"payload":{"kind":"noop","data":[]}}]}"#;
    let (_graph, payloads) = execution_core::dag_loader::parse_dag_from_bytes(json);
    assert!(payloads.contains_key("a"));
}
