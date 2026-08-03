use unicode_width::UnicodeWidthStr;

use crate::canvas::{Grid, Rect};
use crate::error::Result;
use crate::layout::Layout;
use crate::model::{Diagram, EdgeKind, Group, GroupKind, Node, NodeKind, Point, TextItem, TextKind};
use crate::route::{route_diagram, RoutedEdge};
use crate::state::{DiagramState, Status};
use crate::theme::Style;

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
    for cell in &mut grid.cells { cell.style = Style::Background; }
    grid
}

fn paint_groups(grid: &mut Grid, diagram: &Diagram, layout: &Layout) {
    let mut groups = diagram.groups.iter().collect::<Vec<_>>();
    groups.sort_by_key(|group| group_depth(group, diagram));
    for group in groups { paint_group(grid, group, layout); }
}

fn paint_group(grid: &mut Grid, group: &Group, layout: &Layout) {
    let Some(rect) = layout.groups.get(&group.id).copied() else { return; };
    let style = group_style(group.kind);
    grid.draw_box(rect, style, group.dashed);
    grid.write(Point { x: rect.x.saturating_add(2), y: rect.y }, &format!(" {} ", group.label), style);
}

fn paint_routes(grid: &mut Grid, routes: &[RoutedEdge], state: &DiagramState) {
    for route in routes {
        let style = edge_style(route.kind, state.edge(&route.id));
        grid.draw_path(&route.points, style);
        paint_edge_label(grid, route, style);
        paint_arrow(grid, route, style);
    }
}

fn paint_edge_label(grid: &mut Grid, route: &RoutedEdge, style: Style) {
    let Some(anchor) = route.label_at else { return; };
    let width = UnicodeWidthStr::width(route.label.as_str()) as u16;
    let point = Point { x: anchor.x.saturating_sub(width / 2), y: anchor.y.saturating_sub(1) };
    grid.write(point, &route.label, style);
}

fn paint_arrow(grid: &mut Grid, route: &RoutedEdge, style: Style) {
    let Some(pair) = route.points.windows(2).last() else { return; };
    let (point, glyph) = arrow_before(pair[0], pair[1]);
    grid.put(point, glyph, style);
}

fn paint_nodes(grid: &mut Grid, diagram: &Diagram, layout: &Layout, state: &DiagramState) {
    for node in &diagram.nodes {
        let Some(rect) = layout.nodes.get(&node.id).copied() else { continue; };
        let style = node_style(node.kind, state.node(&node.id));
        if node.kind == NodeKind::Decision { paint_decision(grid, node, rect, style); }
        else { paint_box_node(grid, node, rect, style); }
    }
}

fn paint_box_node(grid: &mut Grid, node: &Node, rect: Rect, style: Style) {
    grid.draw_box(rect, style, node.kind == NodeKind::Shell);
    paint_centered_lines(grid, rect, &node.label, style);
}

fn paint_decision(grid: &mut Grid, node: &Node, rect: Rect, style: Style) {
    let label = format!("◇ {}", node.label);
    let width = UnicodeWidthStr::width(label.as_str()) as u16;
    let point = Point { x: rect.x.saturating_add(rect.width.saturating_sub(width) / 2), y: center_y(rect) };
    grid.write(point, &label, style);
}

fn paint_centered_lines(grid: &mut Grid, rect: Rect, label: &str, style: Style) {
    let lines = label.lines().collect::<Vec<_>>();
    let height = u16::try_from(lines.len()).unwrap_or(u16::MAX);
    let start = rect.y.saturating_add(rect.height.saturating_sub(height) / 2);
    for (index, line) in lines.iter().enumerate() {
        paint_centered_line(grid, rect, start, index, line, style);
    }
}

fn paint_centered_line(grid: &mut Grid, rect: Rect, start: u16, index: usize, line: &str, style: Style) {
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
            let y = item.at.y.saturating_add(u16::try_from(index).unwrap_or(u16::MAX));
            grid.write(Point { x: item.at.x, y }, line, text_style(item.kind));
        }
    }
}

fn group_depth(group: &Group, diagram: &Diagram) -> usize {
    let mut depth = 0;
    let mut parent = group.parent.as_deref();
    while let Some(id) = parent {
        depth += 1;
        parent = diagram.groups.iter().find(|item| item.id == id).and_then(|item| item.parent.as_deref());
    }
    depth
}

fn group_style(kind: GroupKind) -> Style {
    match kind { GroupKind::Panel | GroupKind::Lane => Style::Structure, _ => Style::Group }
}

fn node_style(kind: NodeKind, status: Status) -> Style {
    match status {
        Status::Running => Style::Running,
        Status::Succeeded => Style::Success,
        Status::Failed => Style::Failure,
        Status::Idle => kind_style(kind),
    }
}

fn kind_style(kind: NodeKind) -> Style {
    match kind {
        NodeKind::Agent => Style::Agent, NodeKind::Reviewer => Style::Reviewer,
        NodeKind::Decision => Style::Decision, NodeKind::Phase => Style::Phase,
        NodeKind::Input => Style::Input, NodeKind::Output => Style::Output,
        NodeKind::Success => Style::Success, NodeKind::Failure => Style::Failure,
        NodeKind::Shell => Style::Shell, NodeKind::Join => Style::Join,
        NodeKind::Activity => Style::Plain,
    }
}

fn edge_style(kind: EdgeKind, _status: Status) -> Style {
    match kind { EdgeKind::Success => Style::Success, EdgeKind::Failure | EdgeKind::Back => Style::Failure, EdgeKind::Muted => Style::Muted, EdgeKind::Forward => Style::Edge }
}

fn text_style(kind: TextKind) -> Style {
    match kind { TextKind::Title => Style::Title, TextKind::Metric => Style::Metric, TextKind::Dim => Style::Dim, TextKind::Plain => Style::Plain }
}

fn arrow_before(from: Point, to: Point) -> (Point, char) {
    if to.x > from.x { return (Point { x: to.x.saturating_sub(1), y: to.y }, '▶'); }
    if to.x < from.x { return (Point { x: to.x.saturating_add(1), y: to.y }, '◀'); }
    if to.y > from.y { return (Point { x: to.x, y: to.y.saturating_sub(1) }, '▼'); }
    (Point { x: to.x, y: to.y.saturating_add(1) }, '▲')
}

fn clip(value: &str, width: u16) -> String {
    value.chars().take(usize::from(width)).collect()
}

fn center_y(rect: Rect) -> u16 { rect.y.saturating_add(rect.height / 2) }
