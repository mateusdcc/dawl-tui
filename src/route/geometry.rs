use std::collections::HashSet;

pub use super::ports::{port_point, select_ports};
use crate::canvas::Rect;
use crate::layout::Layout;
use crate::model::Point;

pub fn back_path(start: Point, end: Point, floor: u16) -> Vec<Point> {
    let outside = end.x.saturating_sub(2);
    simplify(vec![
        start,
        Point {
            x: start.x,
            y: floor,
        },
        Point {
            x: outside,
            y: floor,
        },
        Point {
            x: outside,
            y: end.y,
        },
        end,
    ])
}

pub fn orthogonalize(start: Point, via: &[Point], end: Point) -> Vec<Point> {
    let mut result = vec![start];
    for next in via.iter().copied().chain(std::iter::once(end)) {
        append_orthogonal(&mut result, next);
    }
    result
}

fn append_orthogonal(points: &mut Vec<Point>, next: Point) {
    let Some(last) = points.last().copied() else {
        points.push(next);
        return;
    };
    if last.x != next.x && last.y != next.y {
        points.push(Point {
            x: last.x,
            y: next.y,
        });
    }
    points.push(next);
}

pub fn simplify(points: Vec<Point>) -> Vec<Point> {
    let mut result = Vec::new();
    for point in points {
        push_simplified(&mut result, point);
    }
    result
}

fn push_simplified(points: &mut Vec<Point>, point: Point) {
    if points.last() == Some(&point) {
        return;
    }
    if points.len() < 2 {
        points.push(point);
        return;
    }
    let a = points[points.len() - 2];
    let b = points[points.len() - 1];
    if collinear(a, b, point) {
        points.pop();
    }
    points.push(point);
}

fn collinear(a: Point, b: Point, c: Point) -> bool {
    (a.x == b.x && b.x == c.x) || (a.y == b.y && b.y == c.y)
}

pub fn label_anchor(points: &[Point], label: &str) -> Option<Point> {
    if label.is_empty() {
        return None;
    }
    let horizontal = points
        .windows(2)
        .filter(|pair| pair[0].y == pair[1].y)
        .max_by_key(|pair| distance(pair[0], pair[1]));
    horizontal
        .or_else(|| {
            points
                .windows(2)
                .max_by_key(|pair| distance(pair[0], pair[1]))
        })
        .map(|pair| midpoint(pair[0], pair[1]))
}

pub fn blocked_cells_for_edge(layout: &Layout, from_id: &str, to_id: &str) -> HashSet<Point> {
    let mut blocked = HashSet::new();
    for (id, rect) in &layout.nodes {
        if id == from_id || id == to_id {
            blocked.extend(interior(rect));
        } else {
            blocked.extend(all_cells(rect));
        }
    }
    blocked
}

fn interior(rect: &Rect) -> Vec<Point> {
    let mut points = Vec::new();
    for y in rect.y.saturating_add(1)..rect.bottom() {
        for x in rect.x.saturating_add(1)..rect.right() {
            points.push(Point { x, y });
        }
    }
    points
}

fn all_cells(rect: &Rect) -> Vec<Point> {
    let mut points = Vec::new();
    for y in rect.y..=rect.bottom() {
        for x in rect.x..=rect.right() {
            points.push(Point { x, y });
        }
    }
    points
}

fn distance(a: Point, b: Point) -> u16 {
    a.x.abs_diff(b.x).saturating_add(a.y.abs_diff(b.y))
}

fn midpoint(a: Point, b: Point) -> Point {
    Point {
        x: a.x.saturating_add(b.x) / 2,
        y: a.y.saturating_add(b.y) / 2,
    }
}
