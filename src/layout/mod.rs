mod automatic;
mod constraints;
mod geometry;
mod groups;
mod shift;
mod topology;

use indexmap::IndexMap;

use crate::canvas::Rect;
use crate::error::Result;
use crate::model::{Diagram, Size};

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
    pub fn new(width: Option<u16>, height: Option<u16>) -> Self {
        Self { width, height }
    }
}

pub fn layout_diagram(diagram: &Diagram, options: &LayoutOptions) -> Result<Layout> {
    diagram.validate()?;
    let mut nodes = automatic::place(diagram)?;
    overlay_explicit_nodes(diagram, &mut nodes);
    let mut groups = groups::layout(diagram, &nodes)?;
    constraints::apply(diagram, &mut nodes, &mut groups)?;
    groups::refresh(diagram, &nodes, &mut groups);
    let size = geometry::canvas_size(diagram, options, &nodes, &groups);
    Ok(Layout {
        nodes,
        groups,
        size,
    })
}

fn overlay_explicit_nodes(diagram: &Diagram, nodes: &mut IndexMap<String, Rect>) {
    for node in &diagram.nodes {
        if let Some(point) = node.at {
            let size = node.size.unwrap_or_else(|| geometry::measure(&node.label));
            nodes.insert(node.id.clone(), geometry::rect(point, size));
        }
    }
}
