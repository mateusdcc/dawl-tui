use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::Result;

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
    #[serde(rename = "node.succeeded")]
    NodeSucceeded { #[serde(alias = "nodeId")] node_id: String },
    #[serde(rename = "node.failed")]
    NodeFailed { #[serde(alias = "nodeId")] node_id: String },
    #[serde(rename = "edge.traversed")]
    EdgeTraversed { #[serde(alias = "edgeId")] edge_id: String },
}

impl DiagramState {
    pub fn apply_json(&mut self, line: &str) -> Result<()> {
        let event: Event = serde_json::from_str(line)?;
        self.apply(event);
        Ok(())
    }

    pub fn node(&self, id: &str) -> Status {
        self.nodes.get(id).copied().unwrap_or_default()
    }

    pub fn edge(&self, id: &str) -> Status {
        self.edges.get(id).copied().unwrap_or_default()
    }

    fn apply(&mut self, event: Event) {
        match event {
            Event::NodeStarted { node_id } => self.set_node(node_id, Status::Running),
            Event::NodeSucceeded { node_id } => self.set_node(node_id, Status::Succeeded),
            Event::NodeFailed { node_id } => self.set_node(node_id, Status::Failed),
            Event::EdgeTraversed { edge_id } => self.set_edge(edge_id, Status::Succeeded),
        }
    }

    fn set_node(&mut self, id: String, status: Status) {
        self.nodes.insert(id, status);
    }

    fn set_edge(&mut self, id: String, status: Status) {
        self.edges.insert(id, status);
    }
}
