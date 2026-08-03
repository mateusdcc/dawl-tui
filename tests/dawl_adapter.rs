use std::path::Path;

use dawl_tui::{load_diagram, DiagramState};

#[test]
fn adapts_current_dawl_graph_shape() {
    let graph = load_diagram(Path::new("fixtures/dawl/approval-graph.json")).unwrap();
    assert_eq!(graph.nodes.len(), 6);
    assert_eq!(graph.schema, "dawl.diagram/v1");
    assert_eq!(graph.nodes[1].group.as_deref(), Some("approval"));
}

#[test]
fn applies_camel_and_snake_case_events() {
    let source = include_str!("../fixtures/dawl/approval-events.ndjson");
    let mut state = DiagramState::default();
    for line in source.lines() {
        state.apply_json(line).unwrap();
    }
    assert_eq!(state.node("developer"), dawl_tui::state::Status::Succeeded);
    assert_eq!(state.edge("e2"), dawl_tui::state::Status::Succeeded);
}
