mod search;

use std::collections::HashSet;

use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::layout::Layout;
use crate::model::{Diagram, Edge, EdgeKind, Point, Port};

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
    let blocked = blocked_cells(layout);
    let mut used = HashSet::new();
    let mut routes = Vec::new();
    for edge in &diagram.edges {
        let route = route_edge(edge, layout, &blocked, &used)?;
        used.extend(expand(&route.points));
        routes.push(route);
    }
    Ok(routes)
}

fn route_edge(edge: &Edge, layout: &Layout, blocked: &HashSet<Point>, used: &HashSet<Point>) -> Result<RoutedEdge> {
    let from = node_rect(layout, &edge.from, edge)?;
    let to = node_rect(layout, &edge.to, edge)?;
    let ports = select_ports(edge, from, to);
    let start = port_point(from, ports.0);
    let end = port_point(to, ports.1);
    let points = path(edge, start, end, layout, blocked, used)?;
    let label_at = label_anchor(&points, &edge.label);
    Ok(RoutedEdge { id: edge.id.clone(), points, label: edge.label.clone(), label_at, kind: edge.kind })
}

fn path(edge: &Edge, start: Point, end: Point, layout: &Layout, blocked: &HashSet<Point>, used: &HashSet<Point>) -> Result<Vec<Point>> {
    if !edge.via.is_empty() { return Ok(simplify(orthogonalize(start, &edge.via, end))); }
    if edge.kind == EdgeKind::Back { return Ok(back_path(start, end)); }
    let finder = Pathfinder::new(layout.size, blocked, used);
    finder.find(start, end).map(simplify).ok_or_else(|| route_error(edge))
}

fn select_ports(edge: &Edge, from: Rect, to: Rect) -> (Port, Port) {
    if let (Some(a), Some(b)) = (edge.from_port, edge.to_port) { return (a, b); }
    if edge.kind == EdgeKind::Back { return (edge.from_port.unwrap_or(Port::South), edge.to_port.unwrap_or(Port::West)); }
    let defaults = directional_ports(from, to, edge.kind);
    (edge.from_port.unwrap_or(defaults.0), edge.to_port.unwrap_or(defaults.1))
}

fn directional_ports(from: Rect, to: Rect, kind: EdgeKind) -> (Port, Port) {
    if kind == EdgeKind::Failure { return (Port::South, Port::West); }
    let dx = i32::from(center_x(to)) - i32::from(center_x(from));
    let dy = i32::from(center_y(to)) - i32::from(center_y(from));
    if dx.abs() >= dy.abs() { horizontal_ports(dx) } else { vertical_ports(dy) }
}

fn horizontal_ports(delta: i32) -> (Port, Port) {
    if delta >= 0 { (Port::East, Port::West) } else { (Port::West, Port::East) }
}

fn vertical_ports(delta: i32) -> (Port, Port) {
    if delta >= 0 { (Port::South, Port::North) } else { (Port::North, Port::South) }
}

fn port_point(rect: Rect, port: Port) -> Point {
    match port {
        Port::North => Point { x: center_x(rect), y: rect.y },
        Port::East => Point { x: rect.right(), y: center_y(rect) },
        Port::South => Point { x: center_x(rect), y: rect.bottom() },
        Port::West => Point { x: rect.x, y: center_y(rect) },
    }
}

fn back_path(start: Point, end: Point) -> Vec<Point> {
    let floor = start.y.max(end.y).saturating_add(3);
    let outside = end.x.saturating_sub(2);
    simplify(vec![start, Point { x: start.x, y: floor }, Point { x: outside, y: floor }, Point { x: outside, y: end.y }, end])
}

fn orthogonalize(start: Point, via: &[Point], end: Point) -> Vec<Point> {
    let mut result = vec![start];
    for next in via.iter().copied().chain(std::iter::once(end)) {
        append_orthogonal(&mut result, next);
    }
    result
}

fn append_orthogonal(points: &mut Vec<Point>, next: Point) {
    let Some(last) = points.last().copied() else { points.push(next); return; };
    if last.x != next.x && last.y != next.y { points.push(Point { x: last.x, y: next.y }); }
    points.push(next);
}

fn simplify(points: Vec<Point>) -> Vec<Point> {
    let mut result = Vec::new();
    for point in points { push_simplified(&mut result, point); }
    result
}

fn push_simplified(points: &mut Vec<Point>, point: Point) {
    if points.last() == Some(&point) { return; }
    if points.len() < 2 { points.push(point); return; }
    let a = points[points.len() - 2];
    let b = points[points.len() - 1];
    if collinear(a, b, point) { points.pop(); }
    points.push(point);
}

fn collinear(a: Point, b: Point, c: Point) -> bool {
    (a.x == b.x && b.x == c.x) || (a.y == b.y && b.y == c.y)
}

fn label_anchor(points: &[Point], label: &str) -> Option<Point> {
    if label.is_empty() { return None; }
    points.windows(2).max_by_key(|pair| distance(pair[0], pair[1])).map(|pair| midpoint(pair[0], pair[1]))
}

fn blocked_cells(layout: &Layout) -> HashSet<Point> {
    layout.nodes.values().flat_map(interior).collect()
}

fn interior(rect: &Rect) -> Vec<Point> {
    let mut points = Vec::new();
    for y in rect.y.saturating_add(1)..rect.bottom() {
        for x in rect.x.saturating_add(1)..rect.right() { points.push(Point { x, y }); }
    }
    points
}

fn expand(points: &[Point]) -> Vec<Point> {
    points.windows(2).flat_map(|pair| segment_points(pair[0], pair[1])).collect()
}

fn segment_points(a: Point, b: Point) -> Vec<Point> {
    if a.x == b.x { return range(a.y, b.y).map(|y| Point { x: a.x, y }).collect(); }
    range(a.x, b.x).map(|x| Point { x, y: a.y }).collect()
}

fn range(a: u16, b: u16) -> std::ops::RangeInclusive<u16> { a.min(b)..=a.max(b) }
fn center_x(rect: Rect) -> u16 { rect.x.saturating_add(rect.width / 2) }
fn center_y(rect: Rect) -> u16 { rect.y.saturating_add(rect.height / 2) }
fn distance(a: Point, b: Point) -> u16 { a.x.abs_diff(b.x).saturating_add(a.y.abs_diff(b.y)) }
fn midpoint(a: Point, b: Point) -> Point { Point { x: a.x.saturating_add(b.x) / 2, y: a.y.saturating_add(b.y) / 2 } }
fn node_rect(layout: &Layout, id: &str, edge: &Edge) -> Result<Rect> { layout.nodes.get(id).copied().ok_or_else(|| route_error(edge)) }
fn route_error(edge: &Edge) -> Error { Error::layout("ROUTE_NOT_FOUND", format!("edge {} cannot be routed", edge.id)).hint("add explicit ports or via points") }
