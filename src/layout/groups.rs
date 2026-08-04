use indexmap::IndexMap;

use super::geometry::{group_box, rect};
use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::model::Diagram;

pub fn layout(diagram: &Diagram, nodes: &IndexMap<String, Rect>) -> Result<IndexMap<String, Rect>> {
    let mut groups = explicit(diagram)?;
    infer_all(diagram, nodes, &mut groups);
    Ok(groups)
}

pub fn refresh(
    diagram: &Diagram,
    nodes: &IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) {
    groups.retain(|id, _| is_explicit(diagram, id));
    infer_all(diagram, nodes, groups);
}

fn explicit(diagram: &Diagram) -> Result<IndexMap<String, Rect>> {
    let mut groups = IndexMap::new();
    for group in &diagram.groups {
        let Some(point) = group.at else { continue };
        let size = group
            .size
            .ok_or_else(|| Error::layout("LAYOUT_GROUP_SIZE", group.id.clone()))?;
        groups.insert(group.id.clone(), rect(point, size));
    }
    Ok(groups)
}

fn infer_all(
    diagram: &Diagram,
    nodes: &IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) {
    for _ in 0..=diagram.groups.len() {
        infer_pass(diagram, nodes, groups);
    }
}

fn infer_pass(
    diagram: &Diagram,
    nodes: &IndexMap<String, Rect>,
    groups: &mut IndexMap<String, Rect>,
) {
    for group in &diagram.groups {
        if groups.contains_key(&group.id) {
            continue;
        }
        let mut children = node_children(diagram, nodes, &group.id);
        children.extend(group_children(diagram, groups, &group.id));
        if let Some(value) = group_box(&children, &group.label) {
            groups.insert(group.id.clone(), value);
        }
    }
}

fn node_children(diagram: &Diagram, nodes: &IndexMap<String, Rect>, id: &str) -> Vec<Rect> {
    diagram
        .nodes
        .iter()
        .filter(|node| node.group.as_deref() == Some(id))
        .filter_map(|node| nodes.get(&node.id).copied())
        .collect()
}

fn group_children(diagram: &Diagram, groups: &IndexMap<String, Rect>, id: &str) -> Vec<Rect> {
    diagram
        .groups
        .iter()
        .filter(|group| group.parent.as_deref() == Some(id))
        .filter_map(|group| groups.get(&group.id).copied())
        .collect()
}

fn is_explicit(diagram: &Diagram, id: &str) -> bool {
    diagram
        .groups
        .iter()
        .any(|group| group.id == id && group.at.is_some())
}
