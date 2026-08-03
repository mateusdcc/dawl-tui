mod event;

use indexmap::IndexMap;

use crate::error::Result;
use crate::model::{Diagram, EdgeKind};

use self::event::{actual_node_id, best_repeat, event_status, node_inside, Event};

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
            Event::EdgeTraversed { edge_id, status } => {
                self.set_edge(edge_id, event_status(status))
            }
        }
    }

    fn apply_node(&mut self, event_id: String, status: Status, graph: Option<&Diagram>) {
        let id = actual_node_id(&event_id, graph);
        self.set_node(id.clone(), status);
        let Some(graph) = graph else {
            return;
        };
        for edge in graph.edges.iter().filter(|edge| edge.to == id) {
            self.set_edge(edge.id.clone(), status);
        }
    }

    fn apply_condition(&mut self, event_id: String, result: bool, graph: Option<&Diagram>) {
        let id = actual_node_id(&event_id, graph);
        let (st, kind) = if result {
            (Status::Succeeded, EdgeKind::Success)
        } else {
            (Status::Failed, EdgeKind::Failure)
        };
        self.set_node(id.clone(), st);
        let Some(graph) = graph else { return };
        for edge in graph
            .edges
            .iter()
            .filter(|e| e.from == id && e.kind == kind)
        {
            self.set_edge(edge.id.clone(), Status::Running);
        }
    }

    fn apply_retry(&mut self, event_id: &str, graph: Option<&Diagram>) {
        let Some(graph) = graph else {
            return;
        };
        let Some(group) = best_repeat(event_id, graph) else {
            return;
        };
        for edge in graph.edges.iter().filter(|e| e.kind == EdgeKind::Back) {
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
