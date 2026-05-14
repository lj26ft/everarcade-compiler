use execution_core::simulation::network_view::NetworkView;
#[test]
fn network_holds_nodes() {
    assert!(NetworkView::default().nodes.is_empty());
}
