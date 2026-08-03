use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::Result;
use crate::model::{Diagram, EdgeKind, GroupKind};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Status {
    Running,
    Succeeded,
    Failed,
    #[default]
    Idle,
}

#[derive(Clone, Debug, Default)]
pub struct DiagramState {
    pub nodes: IndexMap<String, Status>,
    pub edges: IndexMap<String, Status>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Event {
    #[serde(rename = "node.started")]
    NodeStarted { #[serde(alias = "nodeId")] node_id: String },
    #[serde(rename = "node.completed", alias = "node.succeeded")]
    NodeCompleted { #[serde(alias = "nodeId")] node_id: String },
    #[serde(rename = "node.failed")]
    NodeFailed { #[serde(alias = "nodeId")] node_id: String },
    #[serde(rename = "condition.evaluated")]
    Condition { #[serde(alias = "nodeId")] node_id: String, result: bool },
    #[serde(rename = "retry.scheduled")]
    Retry { #[serde(alias = "nodeId")] node_id: String },
    #[serde(rename = "edge.traversed")]
    EdgeTraversed { #[serde(alias = "edgeId")] edge_id: String, status: Option<String> },
}

impl DiagramState {
    pub fn apply_json(&mut self, line: &str) -> Result<()> {
        self.apply_event(serde_json::from_str(line)?, None);
        Ok(())
    }

    pub fn apply_json_with_graph(&mut self, line: &str, graph: &Diagram) -> Result<()> {
        self.apply_event(serde_json::from_str(line)?, Some(graph));
        Ok(())
    }

    pub fn node(&self, id: &str) -> Status {
        self.nodes.get(id).copied().unwrap_or_default()
    }

    pub fn edge(&self, id: &str) -> Status {
        self.edges.get(id).copied().unwrap_or_default()
    }

    fn apply_event(&mut self, event: Event, graph: Option<&Diagram>) {
        match event {
            Event::NodeStarted { node_id } => self.apply_node(node_id, Status::Running, graph),
            Event::NodeCompleted { node_id } => self.apply_node(node_id, Status::Succeeded, graph),
            Event::NodeFailed { node_id } => self.apply_node(node_id, Status::Failed, graph),
            Event::Condition { node_id, result } => self.apply_condition(node_id, result, graph),
            Event::Retry { node_id } => self.apply_retry(&node_id, graph),
            Event::EdgeTraversed { edge_id, status } => self.set_edge(edge_id, event_status(status)),
        }
    }

    fn apply_node(&mut self, event_id: String, status: Status, graph: Option<&Diagram>) {
        let id = actual_node_id(&event_id, graph);
        self.set_node(id.clone(), status);
        let Some(graph) = graph else { return; };
        for edge in graph.edges.iter().filter(|edge| edge.to == id) {
            self.set_edge(edge.id.clone(), status);
        }
    }

    fn apply_condition(&mut self, event_id: String, result: bool, graph: Option<&Diagram>) {
        let id = actual_node_id(&event_id, graph);
        self.set_node(id.clone(), if result { Status::Succeeded } else { Status::Failed });
        let Some(graph) = graph else { return; };
        let kind = if result { EdgeKind::Success } else { EdgeKind::Failure };
        for edge in graph.edges.iter().filter(|edge| edge.from == id && edge.kind == kind) {
            self.set_edge(edge.id.clone(), Status::Running);
        }
    }

    fn apply_retry(&mut self, event_id: &str, graph: Option<&Diagram>) {
        let Some(graph) = graph else { return; };
        let Some(group) = best_repeat(event_id, graph) else { return; };
        for edge in graph.edges.iter().filter(|edge| edge.kind == EdgeKind::Back) {
            if node_inside(&edge.from, &group.id, graph) {
                self.set_edge(edge.id.clone(), Status::Running);
            }
        }
    }

    fn set_node(&mut self, id: String, status: Status) {
        self.nodes.insert(id, status);
    }

    fn set_edge(&mut self, id: String, status: Status) {
        self.edges.insert(id, status);
    }
}

fn actual_node_id(event_id: &str, graph: Option<&Diagram>) -> String {
    let Some(graph) = graph else { return event_id.into(); };
    graph.nodes.iter().map(|node| (&node.id, match_score(event_id, &node.id, &node.label)))
        .max_by_key(|item| item.1).filter(|item| item.1 > 0)
        .map_or_else(|| event_id.into(), |item| item.0.clone())
}

fn best_repeat<'a>(event_id: &str, graph: &'a Diagram) -> Option<&'a crate::model::Group> {
    graph.groups.iter().filter(|group| group.kind == GroupKind::Repeat)
        .max_by_key(|group| match_score(event_id, &group.id, &group.label))
}

fn node_inside(node_id: &str, group_id: &str, graph: &Diagram) -> bool {
    let mut current = graph.nodes.iter().find(|node| node.id == node_id).and_then(|node| node.group.as_deref());
    while let Some(id) = current {
        if id == group_id { return true; }
        current = graph.groups.iter().find(|group| group.id == id).and_then(|group| group.parent.as_deref());
    }
    false
}

fn match_score(event_id: &str, candidate_id: &str, label: &str) -> usize {
    let event = tokens(event_id);
    let overlap = tokens(candidate_id).iter().filter(|token| event.contains(token)).count();
    let label = normalize(label.lines().next().unwrap_or_default());
    overlap + if label.len() > 2 && normalize(event_id).contains(&label) { 20 } else { 0 }
}

fn tokens(value: &str) -> Vec<String> {
    value.to_lowercase().split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty() && *token != "step").map(str::to_owned).collect()
}

fn normalize(value: &str) -> String {
    value.chars().filter(|ch| ch.is_ascii_alphanumeric()).flat_map(char::to_lowercase).collect()
}

fn event_status(status: Option<String>) -> Status {
    match status.as_deref() {
        Some("failed") => Status::Failed,
        Some("succeeded" | "completed") => Status::Succeeded,
        _ => Status::Running,
    }
}
