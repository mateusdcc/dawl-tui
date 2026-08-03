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

#[test]
fn place_constraint_enforces_terminal_separation() {
    let graph = parse("diagram p \"Place\"\nnode a \"A\"\nnode b \"B\"\nplace a before b\n").unwrap();
    let layout = layout_diagram(&graph, &Default::default()).unwrap();
    assert!(layout.nodes["a"].right().saturating_add(4) <= layout.nodes["b"].x);
}

#[test]
fn align_constraint_accepts_compound_groups() {
    let source = r#"
    diagram groups "Groups"
    group one "One"
    group two "Two"
    node a "A" at 5,5 in one
    node b "B" at 30,15 in two
    align vertical one two
    "#;
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    assert_eq!(layout.groups["one"].x, layout.groups["two"].x);
}

#[test]
fn moving_a_group_preserves_child_offsets() {
    let source = r#"
    diagram groups "Groups"
    group one "One"
    node a "A" at 5,5 in one
    node b "B" at 6,10
    place one before b
    "#;
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    let group = layout.groups["one"];
    assert!(group.right().saturating_add(4) <= layout.nodes["b"].x);
    assert!(group.contains(dawl_tui::model::Point { x: layout.nodes["a"].x, y: layout.nodes["a"].y }));
}
