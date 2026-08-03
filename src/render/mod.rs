mod route_paint;
mod style;

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::canvas::{Grid, Rect};
use crate::error::Result;
use crate::layout::Layout;
use crate::model::{Diagram, Group, Node, NodeKind, Point, TextItem};
use crate::route::route_diagram;
use crate::state::DiagramState;
use crate::theme::Style;

use self::route_paint::paint_routes;
use self::style::{group_style, node_style, text_style};

pub fn render_diagram(diagram: &Diagram, layout: &Layout, state: &DiagramState) -> Result<Grid> {
    let routes = route_diagram(diagram, layout)?;
    let mut grid = background(layout);
    paint_groups(&mut grid, diagram, layout);
    paint_routes(&mut grid, &routes, state);
    paint_nodes(&mut grid, diagram, layout, state);
    paint_title(&mut grid, &diagram.title);
    paint_texts(&mut grid, &diagram.texts);
    Ok(grid)
}

fn background(layout: &Layout) -> Grid {
    let mut grid = Grid::new(layout.size.width, layout.size.height);
    for cell in &mut grid.cells {
        cell.style = Style::Background;
    }
    grid
}

fn paint_groups(grid: &mut Grid, diagram: &Diagram, layout: &Layout) {
    let mut groups = diagram.groups.iter().collect::<Vec<_>>();
    groups.sort_by_key(|group| group_depth(group, diagram));
    for group in groups {
        paint_group(grid, group, layout);
    }
}

fn paint_group(grid: &mut Grid, group: &Group, layout: &Layout) {
    let Some(rect) = layout.groups.get(&group.id).copied() else {
        return;
    };
    let style = group_style(group.kind);
    grid.draw_box(rect, style, group.dashed);
    let p = Point {
        x: rect.x.saturating_add(2),
        y: rect.y,
    };
    grid.write(p, &format!(" {} ", group.label), style);
}

fn paint_nodes(grid: &mut Grid, diagram: &Diagram, layout: &Layout, state: &DiagramState) {
    for node in &diagram.nodes {
        let Some(rect) = layout.nodes.get(&node.id).copied() else {
            continue;
        };
        let style = node_style(node.kind, state.node(&node.id));
        if node.kind == NodeKind::Decision {
            paint_decision(grid, node, rect, style);
        } else {
            paint_box_node(grid, node, rect, style);
        }
    }
}

fn paint_box_node(grid: &mut Grid, node: &Node, rect: Rect, style: Style) {
    grid.draw_box(rect, style, node.kind == NodeKind::Shell);
    paint_centered_lines(grid, rect, &node.label, style);
}

fn paint_decision(grid: &mut Grid, node: &Node, rect: Rect, style: Style) {
    grid.draw_diamond(rect, style);
    paint_centered_lines(grid, rect, &node.label, style);
}

fn paint_centered_lines(grid: &mut Grid, rect: Rect, label: &str, style: Style) {
    let lines = label.lines().collect::<Vec<_>>();
    let height = u16::try_from(lines.len()).unwrap_or(u16::MAX);
    let start = rect
        .y
        .saturating_add(rect.height.saturating_sub(height) / 2);
    for (index, line) in lines.iter().enumerate() {
        paint_centered_line(grid, rect, start, index, line, style);
    }
}

fn paint_centered_line(
    grid: &mut Grid,
    rect: Rect,
    start: u16,
    index: usize,
    line: &str,
    style: Style,
) {
    let clipped = clip(line, rect.width.saturating_sub(2));
    let width = UnicodeWidthStr::width(clipped.as_str()) as u16;
    let x = rect.x.saturating_add(rect.width.saturating_sub(width) / 2);
    let y = start.saturating_add(u16::try_from(index).unwrap_or(u16::MAX));
    grid.write(Point { x, y }, &clipped, style);
}

fn paint_title(grid: &mut Grid, title: &str) {
    let width = UnicodeWidthStr::width(title) as u16;
    let x = grid.width.saturating_sub(width) / 2;
    grid.write(Point { x, y: 0 }, title, Style::Title);
}

fn paint_texts(grid: &mut Grid, texts: &[TextItem]) {
    for item in texts {
        for (index, line) in item.text.lines().enumerate() {
            let y = item
                .at
                .y
                .saturating_add(u16::try_from(index).unwrap_or(u16::MAX));
            grid.write(Point { x: item.at.x, y }, line, text_style(item.kind));
        }
    }
}

fn group_depth(group: &Group, diagram: &Diagram) -> usize {
    let mut depth = 0;
    let mut parent = group.parent.as_deref();
    while let Some(id) = parent {
        depth += 1;
        parent = diagram
            .groups
            .iter()
            .find(|item| item.id == id)
            .and_then(|item| item.parent.as_deref());
    }
    depth
}

fn clip(value: &str, width: u16) -> String {
    let mut output = String::new();
    let mut used = 0u16;
    for glyph in value.chars() {
        let glyph_width = UnicodeWidthChar::width(glyph).unwrap_or(0) as u16;
        if used.saturating_add(glyph_width) > width {
            break;
        }
        output.push(glyph);
        used = used.saturating_add(glyph_width);
    }
    output
}
