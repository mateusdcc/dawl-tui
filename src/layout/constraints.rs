use indexmap::IndexMap;

use super::shift::{
    coordinate, empty_alignment, end, entity, order, shift, shift_group, spacing, start,
    unknown_entity, Coordinate,
};
use crate::canvas::Rect;
use crate::error::Result;
use crate::model::{Axis, Constraint, Diagram, Relation};

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
        Constraint::Place {
            first,
            relation,
            second,
        } => place(first, *relation, second, diagram, nodes, groups),
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
    if delta > 0 {
        translate(trail, axis, delta, diagram, nodes, groups)?;
    }
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
    translate(
        id,
        axis,
        i32::from(target) - i32::from(current),
        diagram,
        nodes,
        groups,
    )
}

fn translate(
    id: &str,
    axis: Coordinate,
    delta: i32,
    diagram: &Diagram,
    nodes: &mut IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) -> Result<()> {
    if let Some(rect) = nodes.get_mut(id) {
        shift(rect, axis, delta);
        return Ok(());
    }
    if groups.contains_key(id) {
        shift_group(id, axis, delta, diagram, nodes, groups);
        return Ok(());
    }
    Err(unknown_entity(id))
}
