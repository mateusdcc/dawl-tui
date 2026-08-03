use serde::Deserialize;

use crate::model::{Diagram, GroupKind};
use crate::state::Status;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "node.started")]
    NodeStarted {
        #[serde(alias = "nodeId")]
        node_id: String,
    },
    #[serde(rename = "node.completed", alias = "node.succeeded")]
    NodeCompleted {
        #[serde(alias = "nodeId")]
        node_id: String,
    },
    #[serde(rename = "node.failed")]
    NodeFailed {
        #[serde(alias = "nodeId")]
        node_id: String,
    },
    #[serde(rename = "condition.evaluated")]
    Condition {
        #[serde(alias = "nodeId")]
        node_id: String,
        result: bool,
    },
    #[serde(rename = "retry.scheduled")]
    Retry {
        #[serde(alias = "nodeId")]
        node_id: String,
    },
    #[serde(rename = "edge.traversed")]
    EdgeTraversed {
        #[serde(alias = "edgeId")]
        edge_id: String,
        status: Option<String>,
    },
}

pub fn actual_node_id(event_id: &str, graph: Option<&Diagram>) -> String {
    let Some(graph) = graph else {
        return event_id.into();
    };
    graph
        .nodes
        .iter()
        .map(|node| (&node.id, match_score(event_id, &node.id, &node.label)))
        .max_by_key(|item| item.1)
        .filter(|item| item.1 > 0)
        .map_or_else(|| event_id.into(), |item| item.0.clone())
}

pub fn best_repeat<'a>(event_id: &str, graph: &'a Diagram) -> Option<&'a crate::model::Group> {
    graph
        .groups
        .iter()
        .filter(|group| group.kind == GroupKind::Repeat)
        .max_by_key(|group| match_score(event_id, &group.id, &group.label))
}

pub fn node_inside(node_id: &str, group_id: &str, graph: &Diagram) -> bool {
    let mut current = graph
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .and_then(|node| node.group.as_deref());
    while let Some(id) = current {
        if id == group_id {
            return true;
        }
        current = graph
            .groups
            .iter()
            .find(|group| group.id == id)
            .and_then(|group| group.parent.as_deref());
    }
    false
}

fn match_score(event_id: &str, candidate_id: &str, label: &str) -> usize {
    let event = tokens(event_id);
    let overlap = tokens(candidate_id)
        .iter()
        .filter(|token| event.contains(token))
        .count();
    let label = normalize(label.lines().next().unwrap_or_default());
    let bonus = if label.len() > 2 && normalize(event_id).contains(&label) {
        20
    } else {
        0
    };
    overlap + bonus
}

fn tokens(value: &str) -> Vec<String> {
    value
        .to_lowercase()
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty() && *token != "step")
        .map(str::to_owned)
        .collect()
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

pub fn event_status(status: Option<String>) -> Status {
    match status.as_deref() {
        Some("failed") => Status::Failed,
        Some("active" | "running") => Status::Running,
        _ => Status::Succeeded,
    }
}
