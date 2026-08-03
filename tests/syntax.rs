use dawl_tui::parser::parse;

const SOURCE: &str = r#"
diagram approval "Approval"
viewport 120x40
theme midnight
group issues "parallel(issues)" at 10,3 size 60x28 kind parallel
node input "INPUT\nissues" at 1,12 size 16x5 kind input
node developer "Developer agent" in issues kind agent
node reviewer "Reviewer agent" in issues kind reviewer
decision pass "pass?" in issues
edge e1 input -> developer kind forward
edge e2 developer -> reviewer
edge e3 reviewer -> pass
align vertical developer reviewer pass
place input before developer
"#;

#[test]
fn parses_groups_nodes_edges_and_constraints() {
    let graph = parse(SOURCE).unwrap();
    assert_eq!(graph.title, "Approval");
    assert_eq!(graph.groups[0].id, "issues");
    assert_eq!(graph.edges[0].from, "input");
    assert_eq!(graph.constraints.len(), 2);
    assert_eq!(graph.nodes[0].label, "INPUT\nissues");
}

#[test]
fn reports_the_source_line() {
    let error = parse("diagram x \"X\"\nnode").unwrap_err();
    assert!(error.to_string().contains("line 2"));
}
