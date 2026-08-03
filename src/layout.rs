mod automatic;
mod constraints;

use indexmap::IndexMap;
use unicode_width::UnicodeWidthStr;

use crate::canvas::Rect;
use crate::error::{Error, Result};
use crate::model::{Diagram, Point, Size};

#[derive(Clone, Debug)]
pub struct Layout {
    pub nodes: IndexMap<String, Rect>,
    pub groups: IndexMap<String, Rect>,
    pub size: Size,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LayoutOptions {
    pub width: Option<u16>,
    pub height: Option<u16>,
}

impl LayoutOptions {
    pub fn new(width: Option<u16>, height: Option<u16>) -> Self { Self { width, height } }
}

pub fn layout_diagram(diagram: &Diagram, options: &LayoutOptions) -> Result<Layout> {
    diagram.validate()?;
    let mut nodes = explicit_nodes(diagram);
    automatic::place(diagram, &mut nodes)?;
    let mut groups = layout_groups(diagram, &nodes)?;
    constraints::apply(diagram, &mut nodes, &mut groups)?;
    refresh_inferred_groups(diagram, &nodes, &mut groups);
    let size = canvas_size(diagram, options, &nodes, &groups);
    Ok(Layout { nodes, groups, size })
}

fn explicit_nodes(diagram: &Diagram) -> IndexMap<String, Rect> {
    diagram.nodes.iter().filter_map(|node| {
        node.at.map(|point| (node.id.clone(), rect(point, node.size.unwrap_or_else(|| measure(&node.label)))))
    }).collect()
}

fn layout_groups(diagram: &Diagram, nodes: &IndexMap<String, Rect>) -> Result<IndexMap<String, Rect>> {
    let mut groups = IndexMap::new();
    for group in &diagram.groups {
        if let Some(point) = group.at {
            let size = group.size.ok_or_else(|| Error::layout("LAYOUT_GROUP_SIZE", group.id.clone()))?;
            groups.insert(group.id.clone(), rect(point, size));
        }
    }
    for _ in 0..=diagram.groups.len() { infer_groups(diagram, nodes, &mut groups); }
    Ok(groups)
}

pub(super) fn refresh_inferred_groups(diagram: &Diagram, nodes: &IndexMap<String, Rect>, groups: &mut IndexMap<String, Rect>) {
    groups.retain(|id, _| diagram.groups.iter().any(|group| group.id == *id && group.at.is_some()));
    for _ in 0..=diagram.groups.len() { infer_groups(diagram, nodes, groups); }
}

fn infer_groups(diagram: &Diagram, nodes: &IndexMap<String, Rect>, groups: &mut IndexMap<String, Rect>) {
    for group in &diagram.groups {
        if groups.contains_key(&group.id) { continue; }
        let mut children = node_children(diagram, nodes, &group.id);
        children.extend(group_children(diagram, groups, &group.id));
        if let Some(bounds) = bounds(&children) { groups.insert(group.id.clone(), pad(bounds, 2, 2)); }
    }
}

fn node_children(diagram: &Diagram, nodes: &IndexMap<String, Rect>, id: &str) -> Vec<Rect> {
    diagram.nodes.iter().filter(|node| node.group.as_deref() == Some(id))
        .filter_map(|node| nodes.get(&node.id).copied()).collect()
}

fn group_children(diagram: &Diagram, groups: &IndexMap<String, Rect>, id: &str) -> Vec<Rect> {
    diagram.groups.iter().filter(|group| group.parent.as_deref() == Some(id))
        .filter_map(|group| groups.get(&group.id).copied()).collect()
}

fn bounds(items: &[Rect]) -> Option<Rect> {
    let first = *items.first()?;
    let (mut left, mut top, mut right, mut bottom) = (first.x, first.y, first.right(), first.bottom());
    for item in items.iter().skip(1) {
        left = left.min(item.x); top = top.min(item.y);
        right = right.max(item.right()); bottom = bottom.max(item.bottom());
    }
    Some(Rect { x: left, y: top, width: right - left + 1, height: bottom - top + 1 })
}

fn pad(rect: Rect, x: u16, y: u16) -> Rect {
    Rect { x: rect.x.saturating_sub(x), y: rect.y.saturating_sub(y),
        width: rect.width.saturating_add(x * 2), height: rect.height.saturating_add(y * 2) }
}

fn canvas_size(diagram: &Diagram, options: &LayoutOptions, nodes: &IndexMap<String, Rect>, groups: &IndexMap<String, Rect>) -> Size {
    let all = nodes.values().chain(groups.values()).copied().collect::<Vec<_>>();
    let extent = bounds(&all).unwrap_or_default();
    let width = options.width.unwrap_or(diagram.viewport.width.max(extent.right().saturating_add(2)));
    let height = options.height.unwrap_or(diagram.viewport.height.max(extent.bottom().saturating_add(2)));
    Size { width: width.max(20), height: height.max(8) }
}

fn rect(point: Point, size: Size) -> Rect { Rect { x: point.x, y: point.y, width: size.width, height: size.height } }

pub(super) fn measure(label: &str) -> Size {
    let width = label.lines().map(UnicodeWidthStr::width).max().unwrap_or(1) as u16;
    let height = label.lines().count().max(1) as u16;
    Size { width: width.saturating_add(4), height: height.saturating_add(2) }
}
