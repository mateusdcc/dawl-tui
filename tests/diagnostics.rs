use dawl_tui::layout::layout_diagram;
use dawl_tui::parser::parse;
use dawl_tui::route::route_diagram;

#[test]
fn impossible_route_names_the_edge_and_recovery_action() {
    let source = r#"
    diagram blocked "Blocked"
    viewport 12x8
    node a "A" at 1,2 size 5x3
    node wall "Wall" at 6,0 size 5x8
    node b "B" at 7,2 size 4x3
    edge important a -> b
    "#;
    let diagram = parse(source).unwrap();
    let layout = layout_diagram(&diagram, &Default::default()).unwrap();
    let error = route_diagram(&diagram, &layout).unwrap_err();
    assert_eq!(error.code, "ROUTE_NOT_FOUND");
    assert!(error.to_string().contains("important"));
    assert!(error.to_string().contains("explicit ports or via points"));
}
