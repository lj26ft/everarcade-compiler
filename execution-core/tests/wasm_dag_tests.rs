use execution_core::dag::{ExecutionGraph, ExecutionNode};

#[test]
fn dag_reordered_input_same_output() {
    let a = ExecutionNode {
        id: "a".into(),
        deps: vec![],
    };
    let b = ExecutionNode {
        id: "b".into(),
        deps: vec!["a".into()],
    };
    let mut g1 = ExecutionGraph::new();
    g1.add_node(a.clone());
    g1.add_node(b.clone());
    let mut g2 = ExecutionGraph::new();
    g2.add_node(b);
    g2.add_node(a);
    assert_eq!(
        g1.topo_sort_checked().unwrap(),
        g2.topo_sort_checked().unwrap()
    );
}
