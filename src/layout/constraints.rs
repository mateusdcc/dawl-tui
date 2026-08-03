use indexmap::IndexMap;

use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::model::{Axis, Constraint, Diagram, Direction, Relation};

#[derive(Clone, Copy)]
enum Coordinate {
    X,
    Y,
}

pub(super) fn apply(
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) -> Result<()> {
    for constraint in &diagram.constraints {
        apply_one(constraint, diagram, nodes, groups)?;
        super::refresh_inferred_groups(diagram, nodes, groups);
    }
    Ok(())
}

fn apply_one(
    constraint: &Constraint,
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) -> Result<()> {
    match constraint {
        Constraint::Align { axis, ids } => align(*axis, ids, diagram, nodes, groups),
        Constraint::Place { first, relation, second } => {
            place(first, *relation, second, diagram, nodes, groups)
        }
    }
}

fn align(
    axis: Axis,
    ids: &[String],
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) -> Result<()> {
    let first = ids.first().ok_or_else(empty_alignment)?;
    let target = start(entity(nodes, groups, first)?, coordinate(axis));
    for id in ids.iter().skip(1) {
        move_start(id, coordinate(axis), target, diagram, nodes, groups)?;
    }
    Ok(())
}

fn place(
    first: &str,
    relation: Relation,
    second: &str,
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) -> Result<()> {
    let (lead, trail, axis) = order(first, relation, second, diagram.direction);
    let lead_rect = entity(nodes, groups, lead)?;
    let trail_rect = entity(nodes, groups, trail)?;
    let required = end(lead_rect, axis).saturating_add(spacing(axis));
    let delta = i32::from(required).saturating_sub(i32::from(start(trail_rect, axis)));
    if delta > 0 { translate(trail, axis, delta, diagram, nodes, groups)?; }
    Ok(())
}

fn move_start(
    id: &str,
    axis: Coordinate,
    target: u16,
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) -> Result<()> {
    let current = start(entity(nodes, groups, id)?, axis);
    translate(id, axis, i32::from(target) - i32::from(current), diagram, nodes, groups)
}

fn translate(
    id: &str,
    axis: Coordinate,
    delta: i32,
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) -> Result<()> {
    if let Some(rect) = nodes.get_mut(id) { shift(rect, axis, delta); return Ok(()); }
    if groups.contains_key(id) { shift_group(id, axis, delta, diagram, nodes, groups); return Ok(()); }
    Err(unknown_entity(id))
}

fn shift_group(
    id: &str,
    axis: Coordinate,
    delta: i32,
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) {
    for node in &diagram.nodes {
        if node.group.as_deref().is_some_and(|group| inside(group, id, diagram)) {
            if let Some(rect) = nodes.get_mut(&node.id) { shift(rect, axis, delta); }
        }
    }
    for group in &diagram.groups {
        let descendant = group.id == id
            || group.parent.as_deref().is_some_and(|parent| inside(parent, id, diagram));
        if !descendant { continue; }
        if let Some(rect) = groups.get_mut(&group.id) { shift(rect, axis, delta); }
    }
}

fn inside(mut current: &str, target: &str, diagram: &Diagram) -> bool {
    loop {
        if current == target { return true; }
        let Some(parent) = diagram.groups.iter().find(|group| group.id == current)
            .and_then(|group| group.parent.as_deref()) else { return false; };
        current = parent;
    }
}

fn order<'a>(
    first: &'a str,
    relation: Relation,
    second: &'a str,
    direction: Direction,
) -> (&'a str, &'a str, Coordinate) {
    match relation {
        Relation::Before => (first, second, flow_axis(direction)),
        Relation::After => (second, first, flow_axis(direction)),
        Relation::Above => (first, second, Coordinate::Y),
        Relation::Below => (second, first, Coordinate::Y),
    }
}

fn flow_axis(direction: Direction) -> Coordinate {
    match direction { Direction::Right => Coordinate::X, Direction::Down => Coordinate::Y }
}

fn coordinate(axis: Axis) -> Coordinate {
    match axis { Axis::Horizontal => Coordinate::Y, Axis::Vertical => Coordinate::X }
}

fn entity(nodes: &IndexMap<String, Rect>, groups: &IndexMap<String, Rect>, id: &str) -> Result<Rect> {
    nodes.get(id).or_else(|| groups.get(id)).copied().ok_or_else(|| unknown_entity(id))
}

fn start(rect: Rect, axis: Coordinate) -> u16 {
    match axis { Coordinate::X => rect.x, Coordinate::Y => rect.y }
}

fn end(rect: Rect, axis: Coordinate) -> u16 {
    match axis { Coordinate::X => rect.right(), Coordinate::Y => rect.bottom() }
}

fn spacing(axis: Coordinate) -> u16 {
    match axis { Coordinate::X => 4, Coordinate::Y => 2 }
}

fn shift(rect: &mut Rect, axis: Coordinate, delta: i32) {
    match axis {
        Coordinate::X => rect.x = shifted(rect.x, delta),
        Coordinate::Y => rect.y = shifted(rect.y, delta),
    }
}

fn shifted(value: u16, delta: i32) -> u16 {
    (i32::from(value) + delta).clamp(0, i32::from(u16::MAX)) as u16
}

fn empty_alignment() -> Error {
    Error::layout("LAYOUT_ALIGN", "alignment has no entity")
}

fn unknown_entity(id: &str) -> Error {
    Error::layout("LAYOUT_CONSTRAINT", format!("unknown constrained entity {id}"))
}
