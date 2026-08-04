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
    let graph =
        parse("diagram p \"Place\"\nnode a \"A\"\nnode b \"B\"\nplace a before b\n").unwrap();
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
    assert!(group.contains(dawl_tui::model::Point {
        x: layout.nodes["a"].x,
        y: layout.nodes["a"].y
    }));
}

#[test]
fn forward_cycle_requires_an_explicit_back_edge() {
    let source =
        "diagram cycle \"Cycle\"\nnode a \"A\"\nnode b \"B\"\nedge ab a -> b\nedge ba b -> a\n";
    let error = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap_err();
    assert_eq!(error.code, "LAYOUT_CYCLE");
}

#[test]
fn automatic_layers_separate_variable_height_nodes() {
    let source = "diagram height \"Height\"\nnode a \"A\\nA\\nA\"\nnode b \"B\\nB\"\n";
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    assert!(layout.nodes["a"].bottom().saturating_add(2) <= layout.nodes["b"].y);
}

#[test]
fn barycenter_sweep_reduces_a_two_edge_crossing() {
    let source = "diagram cross \"Cross\"\nnode a \"A\"\nnode b \"B\"\nnode c \"C\"\nnode d \"D\"\nedge ad a -> d\nedge bc b -> c\n";
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    assert!(layout.nodes["d"].y < layout.nodes["c"].y);
}

#[test]
fn automatic_nodes_use_intrinsic_dimensions_and_padding() {
    let graph = parse("diagram sized \"Sized\"\nnode a \"wide label\\nline two\"\n").unwrap();
    let layout = layout_diagram(&graph, &Default::default()).unwrap();
    let node = layout.nodes["a"];
    assert_eq!((node.width, node.height), (14, 4));
    assert!(node.x >= 2 && node.y >= 2);
    assert!(layout.size.width >= node.right() + 3);
    assert!(layout.size.height >= node.bottom() + 3);
}

#[test]
fn compound_topology_separates_groups_and_contains_children() {
    let source = r#"
    diagram compound "Compound"
    group left "Left lane"
    group right "Right lane"
    node a "A" in left
    node b "B" in right
    edge flow a -> b
    "#;
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    assert!(layout.groups["left"].right() < layout.groups["right"].x);
    assert!(layout.groups["left"].contains(dawl_tui::model::Point {
        x: layout.nodes["a"].x,
        y: layout.nodes["a"].y,
    }));
    assert!(layout.groups["right"].contains(dawl_tui::model::Point {
        x: layout.nodes["b"].x,
        y: layout.nodes["b"].y,
    }));
}

#[test]
fn automatic_rank_and_node_spacing_are_consistent() {
    let source = r#"
    diagram spacing "Spacing"
    node root "Root"
    node one "One"
    node two "Two"
    edge first root -> one
    edge second root -> two
    "#;
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    let (one, two) = (layout.nodes["one"], layout.nodes["two"]);
    assert!(layout.nodes["root"].right().saturating_add(8) <= one.x);
    assert!(one.bottom().saturating_add(4) <= two.y || two.bottom().saturating_add(4) <= one.y);
}

#[test]
fn canvas_expands_for_free_text_without_a_viewport() {
    let source = "diagram text \"Dynamic title\"\ntext note \"far note\" at 80,30\n";
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    assert!(layout.size.width >= 90);
    assert!(layout.size.height >= 33);
}

#[test]
fn nested_groups_preserve_the_diagram_margin() {
    let source = r#"
    diagram nested "Nested"
    group outer "Outer"
    group inner "Inner" in outer
    node child "Child" in inner
    "#;
    let layout = layout_diagram(&parse(source).unwrap(), &Default::default()).unwrap();
    assert!(layout.groups["outer"].x >= 2);
    assert!(layout.groups["outer"].y >= 2);
}
