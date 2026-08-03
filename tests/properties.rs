use dawl_tui::layout::layout_diagram;
use dawl_tui::model::Point;
use dawl_tui::parser::parse;
use dawl_tui::route::route_diagram;

#[test]
fn generated_chains_are_deterministic_and_orthogonal() {
    for length in 2..18 {
        let diagram = parse(&chain_source(length)).unwrap();
        let first = layout_diagram(&diagram, &Default::default()).unwrap();
        let second = layout_diagram(&diagram, &Default::default()).unwrap();
        assert_eq!(first.nodes, second.nodes);
        let routes = route_diagram(&diagram, &first).unwrap();
        assert!(routes.iter().flat_map(|route| route.points.windows(2))
            .all(|pair| orthogonal(pair[0], pair[1])));
    }
}

#[test]
fn canonical_json_round_trips_without_losing_ids() {
    let diagram = parse(&chain_source(7)).unwrap();
    let json = serde_json::to_string(&diagram).unwrap();
    let decoded: dawl_tui::Diagram = serde_json::from_str(&json).unwrap();
    decoded.validate().unwrap();
    assert_eq!(diagram.nodes.iter().map(|node| &node.id).collect::<Vec<_>>(),
        decoded.nodes.iter().map(|node| &node.id).collect::<Vec<_>>());
}

fn chain_source(length: usize) -> String {
    let mut source = String::from("diagram generated \"Generated\"\nviewport 500x80\n");
    for index in 0..length { source.push_str(&format!("node n{index} \"N{index}\"\n")); }
    for index in 1..length { source.push_str(&format!("edge e{index} n{} -> n{index}\n", index - 1)); }
    source
}

fn orthogonal(a: Point, b: Point) -> bool { a.x == b.x || a.y == b.y }
