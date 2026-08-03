use unicode_width::UnicodeWidthStr;

use crate::canvas::Grid;
use crate::model::Point;
use crate::route::RoutedEdge;
use crate::state::DiagramState;
use crate::theme::Style;

use super::style::edge_style;

pub fn paint_routes(grid: &mut Grid, routes: &[RoutedEdge], state: &DiagramState) {
    for route in routes {
        let style = edge_style(route.kind, state.edge(&route.id));
        grid.draw_path(&route.points, style);
        paint_edge_label(grid, route, style);
        paint_arrow(grid, route, style);
    }
}

fn paint_edge_label(grid: &mut Grid, route: &RoutedEdge, style: Style) {
    let Some(anchor) = route.label_at else {
        return;
    };
    let width = UnicodeWidthStr::width(route.label.as_str()) as u16;
    let point = Point {
        x: anchor.x.saturating_sub(width / 2),
        y: anchor.y.saturating_sub(1),
    };
    grid.write(point, &route.label, style);
}

fn paint_arrow(grid: &mut Grid, route: &RoutedEdge, style: Style) {
    let Some(pair) = route.points.windows(2).last() else {
        return;
    };
    let (point, glyph) = arrow_before(pair[0], pair[1]);
    grid.put(point, glyph, style);
}

fn pt(x: u16, y: u16) -> Point {
    Point { x, y }
}

fn arrow_before(from: Point, to: Point) -> (Point, char) {
    if to.x > from.x {
        (pt(to.x.saturating_sub(1), to.y), '▶')
    } else if to.x < from.x {
        (pt(to.x.saturating_add(1), to.y), '◀')
    } else if to.y > from.y {
        (pt(to.x, to.y.saturating_sub(1)), '▼')
    } else {
        (pt(to.x, to.y.saturating_add(1)), '▲')
    }
}
