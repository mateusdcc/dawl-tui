use dagre::graph::{Graph, GraphOptions};
use dagre::{EdgeLabel, LayoutOptions, NodeLabel, RankDir};
use indexmap::IndexMap;

use super::geometry::{
    measure, text_size, CANVAS_PADDING_X, CANVAS_PADDING_Y, GROUP_PADDING_X, GROUP_PADDING_Y,
};
use super::shift::{shift, Coordinate};
use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::model::{Diagram, Direction, Edge, EdgeKind, Node};

pub type LayoutGraph = Graph<NodeLabel, EdgeLabel>;

pub fn layout(diagram: &Diagram) -> LayoutGraph {
    let mut graph = new_graph();
    add_groups(diagram, &mut graph);
    add_nodes(diagram, &mut graph);
    add_parents(diagram, &mut graph);
    add_edges(diagram, &mut graph);
    dagre::layout(&mut graph, Some(options(diagram.direction)));
    graph
}

pub fn node_rects(diagram: &Diagram, graph: &LayoutGraph) -> Result<IndexMap<String, Rect>> {
    let mut nodes = IndexMap::new();
    for node in &diagram.nodes {
        let label = graph.node(&node.id).ok_or_else(|| missing(&node.id))?;
        nodes.insert(node.id.clone(), output_rect(label, &node.id)?);
    }
    normalize(diagram, &mut nodes);
    Ok(nodes)
}

fn new_graph() -> LayoutGraph {
    Graph::with_options(GraphOptions {
        directed: true,
        multigraph: true,
        compound: true,
    })
}

fn add_groups(diagram: &Diagram, graph: &mut LayoutGraph) {
    for group in &diagram.groups {
        graph.set_node(group.id.clone(), Some(NodeLabel::default()));
    }
}

fn add_nodes(diagram: &Diagram, graph: &mut LayoutGraph) {
    for node in &diagram.nodes {
        graph.set_node(node.id.clone(), Some(node_label(node)));
    }
}

fn add_parents(diagram: &Diagram, graph: &mut LayoutGraph) {
    for group in &diagram.groups {
        if let Some(parent) = &group.parent {
            graph.set_parent(&group.id, Some(parent));
        }
    }
    for node in &diagram.nodes {
        if let Some(parent) = &node.group {
            graph.set_parent(&node.id, Some(parent));
        }
    }
}

fn add_edges(diagram: &Diagram, graph: &mut LayoutGraph) {
    for edge in diagram
        .edges
        .iter()
        .filter(|edge| edge.kind != EdgeKind::Back)
    {
        graph.set_edge(
            edge.from.clone(),
            edge.to.clone(),
            Some(edge_label(edge)),
            Some(&edge.id),
        );
    }
}

fn node_label(node: &Node) -> NodeLabel {
    let size = node.size.unwrap_or_else(|| measure(&node.label));
    NodeLabel {
        width: f64::from(size.width),
        height: f64::from(size.height),
        ..Default::default()
    }
}

fn edge_label(edge: &Edge) -> EdgeLabel {
    let size = text_size(&edge.label);
    EdgeLabel {
        width: if edge.label.is_empty() {
            0.0
        } else {
            f64::from(size.width)
        },
        height: if edge.label.is_empty() {
            0.0
        } else {
            f64::from(size.height)
        },
        ..Default::default()
    }
}

fn options(direction: Direction) -> LayoutOptions {
    let (rankdir, nodesep, ranksep) = spacing(direction);
    LayoutOptions {
        rankdir,
        nodesep,
        ranksep,
        edgesep: 2.0,
        tie_keep_first: true,
        ..Default::default()
    }
}

fn spacing(direction: Direction) -> (RankDir, f64, f64) {
    match direction {
        Direction::Right => (RankDir::LR, 4.0, 8.0),
        Direction::Down => (RankDir::TB, 6.0, 4.0),
    }
}

fn output_rect(label: &NodeLabel, id: &str) -> Result<Rect> {
    let x = label.x.ok_or_else(|| missing(id))? - label.width / 2.0;
    let y = label.y.ok_or_else(|| missing(id))? - label.height / 2.0;
    Ok(Rect {
        x: cell(x),
        y: cell(y),
        width: cell(label.width).max(1),
        height: cell(label.height).max(1),
    })
}

fn normalize(diagram: &Diagram, nodes: &mut IndexMap<String, Rect>) {
    let x = nodes.values().map(|rect| rect.x).min().unwrap_or(0);
    let y = nodes.values().map(|rect| rect.y).min().unwrap_or(0);
    let depth = max_group_depth(diagram);
    let margin_x = CANVAS_PADDING_X.saturating_add(GROUP_PADDING_X.saturating_mul(depth));
    let margin_y = CANVAS_PADDING_Y.saturating_add(GROUP_PADDING_Y.saturating_mul(depth));
    let dx = i32::from(margin_x) - i32::from(x);
    let dy = i32::from(margin_y) - i32::from(y);
    for rect in nodes.values_mut() {
        shift(rect, Coordinate::X, dx);
        shift(rect, Coordinate::Y, dy);
    }
}

fn max_group_depth(diagram: &Diagram) -> u16 {
    diagram
        .groups
        .iter()
        .map(|group| group_depth(&group.id, diagram))
        .max()
        .unwrap_or(0)
}

fn group_depth(id: &str, diagram: &Diagram) -> u16 {
    let mut depth = 1u16;
    let mut current = id;
    while let Some(parent) = diagram
        .groups
        .iter()
        .find(|group| group.id == current)
        .and_then(|group| group.parent.as_deref())
    {
        depth = depth.saturating_add(1);
        current = parent;
    }
    depth
}

fn cell(value: f64) -> u16 {
    value.round().clamp(0.0, f64::from(u16::MAX)) as u16
}

fn missing(id: &str) -> Error {
    Error::layout("LAYOUT_ENGINE", format!("layout engine omitted node {id}"))
}
