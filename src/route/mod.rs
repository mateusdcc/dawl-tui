mod geometry;
mod ports;
mod search;

use std::collections::HashSet;

use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::layout::Layout;
use crate::model::{Diagram, Edge, EdgeKind, Point};

use self::geometry::{
    back_path, blocked_cells_for_edge, expand, label_anchor, orthogonalize, port_point,
    select_ports, simplify,
};
use search::Pathfinder;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedEdge {
    pub id: String,
    pub points: Vec<Point>,
    pub label: String,
    pub label_at: Option<Point>,
    pub kind: EdgeKind,
}

pub fn route_diagram(diagram: &Diagram, layout: &Layout) -> Result<Vec<RoutedEdge>> {
    let mut used = HashSet::new();
    let mut routes = Vec::new();
    for edge in &diagram.edges {
        let route = route_edge(edge, layout, &used)?;
        used.extend(expand(&route.points));
        routes.push(route);
    }
    Ok(routes)
}

fn route_edge(edge: &Edge, layout: &Layout, used: &HashSet<Point>) -> Result<RoutedEdge> {
    let blocked = blocked_cells_for_edge(layout, &edge.from, &edge.to);
    let from = node_rect(layout, &edge.from, edge)?;
    let to = node_rect(layout, &edge.to, edge)?;
    let ports = select_ports(edge, from, to);
    let start = port_point(from, ports.0);
    let end = port_point(to, ports.1);
    let points = path(edge, start, end, layout, &blocked, used)?;
    let label_at = label_anchor(&points, &edge.label);
    Ok(RoutedEdge {
        id: edge.id.clone(),
        points,
        label: edge.label.clone(),
        label_at,
        kind: edge.kind,
    })
}

fn path(
    edge: &Edge,
    start: Point,
    end: Point,
    layout: &Layout,
    blocked: &HashSet<Point>,
    used: &HashSet<Point>,
) -> Result<Vec<Point>> {
    if !edge.via.is_empty() {
        return Ok(simplify(orthogonalize(start, &edge.via, end)));
    }
    if edge.kind == EdgeKind::Back {
        return Ok(back_path(start, end));
    }
    let finder = Pathfinder::new(layout.size, blocked, used);
    finder
        .find(start, end)
        .map(simplify)
        .ok_or_else(|| route_error(edge))
}

fn node_rect(layout: &Layout, id: &str, edge: &Edge) -> Result<Rect> {
    layout
        .nodes
        .get(id)
        .copied()
        .ok_or_else(|| route_error(edge))
}

fn route_error(edge: &Edge) -> Error {
    Error::layout(
        "ROUTE_NOT_FOUND",
        format!("edge {} cannot be routed", edge.id),
    )
    .hint("add explicit ports or via points")
}
