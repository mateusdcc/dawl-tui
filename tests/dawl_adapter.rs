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

#[test]
fn accepts_all_node_and_group_kinds_emitted_by_dawl() {
    let source = r#"{
      "title":"Current DAWL kinds",
      "nodes":[
        {"id":"fork","label":"parallel","kind":"fork"},
        {"id":"value","label":"value","kind":"value"},
        {"id":"ret","label":"return result","kind":"return"},
        {"id":"join","label":"results","kind":"join"}
      ],
      "edges":[
        {"id":"e1","from":"fork","to":"value"},
        {"id":"e2","from":"value","to":"ret"},
        {"id":"e3","from":"ret","to":"join"}
      ],
      "groups":[{"id":"fn","label":"approve","kind":"function"}]
    }"#;
    let graph = dawl_tui::input::parse_source(Path::new("graph.json"), source).unwrap();
    assert_eq!(graph.nodes.len(), 4);
    assert_eq!(graph.groups.len(), 1);
}
