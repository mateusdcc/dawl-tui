use dawl_tui::model::{Diagram, Edge, EdgeKind, Node, NodeKind};

fn node(id: &str) -> Node {
    Node { id: id.into(), label: id.into(), kind: NodeKind::Activity, group: None, at: None, size: None }
}

#[test]
fn rejects_an_edge_with_an_unknown_endpoint() {
    let mut diagram = Diagram { title: "x".into(), ..Default::default() };
    diagram.nodes.push(node("a"));
    diagram.edges.push(Edge {
        id: "e".into(), from: "a".into(), to: "missing".into(),
        label: String::new(), kind: EdgeKind::Forward, via: vec![],
        from_port: None, to_port: None,
    });
    let error = diagram.validate().unwrap_err();
    assert_eq!(error.code, "MODEL_UNKNOWN_NODE");
}

#[test]
fn rejects_duplicate_node_ids() {
    let mut diagram = Diagram { title: "x".into(), ..Default::default() };
    diagram.nodes = vec![node("a"), node("a")];
    let error = diagram.validate().unwrap_err();
    assert_eq!(error.code, "MODEL_DUPLICATE_ID");
}
