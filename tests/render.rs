use dawl_tui::layout::layout_diagram;
use dawl_tui::parser::parse;
use dawl_tui::render::render_diagram;
use dawl_tui::state::DiagramState;

#[test]
fn paints_nested_groups_semantic_paths_and_metrics() {
    let diagram = parse(RENDER_SOURCE).unwrap();
    let layout = layout_diagram(&diagram, &Default::default()).unwrap();
    let grid = render_diagram(&diagram, &layout, &DiagramState::default()).unwrap();
    let text = plain(&grid);
    assert_labels(&text);
}

fn assert_labels(text: &str) {
    for label in ["developIssuesUntilApproved: full agent flow", "parallel(issues)",
        "developUntilApproved", "Developer agent", "YES", "NO", "findings",
        "Worst case: 2NM + 2M + 1 agents"] {
        assert!(text.contains(label), "missing {label}");
    }
}

#[test]
fn runtime_state_changes_style_without_moving_nodes() {
    let source = r#"
    diagram state "State"
    viewport 40x12
    node agent "Agent" at 3,4 size 11x3 kind agent
    "#;
    let diagram = parse(source).unwrap();
    let layout = layout_diagram(&diagram, &Default::default()).unwrap();
    let mut state = DiagramState::default();
    state.apply_json(r#"{"type":"node.started","nodeId":"agent"}"#).unwrap();
    let grid = render_diagram(&diagram, &layout, &state).unwrap();
    assert_eq!(grid.cell(3, 4).unwrap().style, dawl_tui::theme::Style::Running);
    assert_eq!(layout.nodes["agent"].x, 3);
}

fn plain(grid: &dawl_tui::canvas::Grid) -> String {
    (0..grid.height).map(|y| {
        (0..grid.width).map(|x| grid.visible_char(x, y)).collect::<String>()
    }).collect::<Vec<_>>().join("\n")
}

const RENDER_SOURCE: &str = r#"
    diagram approval "developIssuesUntilApproved: full agent flow"
    viewport 90x28
    group issues "parallel(issues)" at 2,3 size 55x18 kind parallel
    group repeat "developUntilApproved" at 16,6 size 35x11 kind repeat in issues dashed
    node developer "Developer agent" at 19,9 size 14x3 kind agent in repeat
    node reviewer "Reviewer agent" at 36,9 size 13x3 kind reviewer in repeat
    decision pass "pass?" at 51,9 size 7x3 in repeat
    node approved "YES" at 63,7 size 7x3 kind success
    node failed "failed review" at 42,14 size 15x3 kind failure in repeat
    edge work developer -> reviewer
    edge review reviewer -> pass
    edge yes pass -> approved kind success label "YES"
    edge no pass -> failed kind failure label "NO"
    edge retry failed -> developer kind back label "findings"
    text metric "Worst case: 2NM + 2M + 1 agents" at 3,24 kind metric
    "#;
