use dawl_tui::layout::{layout_diagram, LayoutOptions};
use dawl_tui::parser::parse;

#[test]
fn automatic_chain_ranks_left_to_right() {
    let source = r#"
    diagram chain "Chain"
    node a "A"
    node b "B"
    node c "C"
    edge ab a -> b
    edge bc b -> c
    "#;
    let graph = parse(source).unwrap();
    let layout = layout_diagram(&graph, &LayoutOptions::default()).unwrap();
    assert!(layout.nodes["a"].x < layout.nodes["b"].x);
    assert!(layout.nodes["b"].x < layout.nodes["c"].x);
}

#[test]
fn explicit_layout_is_deterministic() {
    let source = r#"
    diagram fixed "Fixed"
    viewport 80x20
    group g "Group" at 5,2 size 40x15 kind panel
    node a "A" at 8,7 size 9x3 in g
    "#;
    let graph = parse(source).unwrap();
    let first = layout_diagram(&graph, &Default::default()).unwrap();
    let second = layout_diagram(&graph, &Default::default()).unwrap();
    assert_eq!(first.nodes["a"], second.nodes["a"]);
    assert!(first.groups["g"].contains(dawl_tui::model::Point { x: 8, y: 7 }));
}
