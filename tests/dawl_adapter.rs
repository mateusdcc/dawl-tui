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

#[test]
fn projects_current_dawl_condition_and_retry_events() {
    let graph = dawl_tui::input::parse_source(Path::new("graph.json"), event_graph()).unwrap();
    let mut state = DiagramState::default();
    let events = [
        r#"{"type":"node.completed","nodeId":"flow.approval.developer"}"#,
        r#"{"type":"condition.evaluated","nodeId":"flow.approval.pass","result":false}"#,
        r#"{"type":"retry.scheduled","nodeId":"flow.approval.repeat"}"#,
    ];
    for event in events {
        apply(&mut state, &graph, event);
    }
    assert_eq!(state.node("developer"), dawl_tui::state::Status::Succeeded);
    assert_eq!(state.node("pass"), dawl_tui::state::Status::Failed);
    assert_eq!(state.edge("no"), dawl_tui::state::Status::Running);
    assert_eq!(state.edge("retry"), dawl_tui::state::Status::Running);
}

fn apply(state: &mut DiagramState, graph: &dawl_tui::Diagram, event: &str) {
    state.apply_json_with_graph(event, graph).unwrap();
}

fn event_graph() -> &'static str {
    r#"{
      "title":"Events",
      "nodes":[
        {"id":"developer","label":"Developer","kind":"agent","groupId":"approval"},
        {"id":"pass","label":"pass?","kind":"decision","groupId":"approval"},
        {"id":"failed","label":"failed review","kind":"failure","groupId":"approval"}
      ],
      "edges":[
        {"id":"to-pass","from":"developer","to":"pass"},
        {"id":"no","from":"pass","to":"failed","kind":"failure"},
        {"id":"retry","from":"failed","to":"developer","kind":"back"}
      ],
      "groups":[{"id":"approval","label":"repeat M fresh","kind":"repeat"}]
    }"#
}
