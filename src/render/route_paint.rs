use unicode_width::UnicodeWidthStr;

use crate::canvas::{ArrowDirection, Grid, LineLayer};
use crate::model::Point;
use crate::route::RoutedEdge;
use crate::state::DiagramState;
use crate::theme::Style;

use super::style::edge_style;

pub fn paint_routes(grid: &mut Grid, routes: &[RoutedEdge], state: &DiagramState) {
    for route in routes {
        let style = edge_style(route.kind, state.edge(&route.id));
        let arrows = arrow_markers(grid, route);
        grid.draw_path(&route.points, style);
        for marker in arrows {
            grid.arrow(marker.point, marker.direction, style);
        }
        paint_edge_label(grid, route, style);
    }
}

const ARROW_INTERVAL: u16 = 8;
const END_CLEARANCE: u16 = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ArrowMarker {
    point: Point,
    direction: ArrowDirection,
}

fn paint_edge_label(grid: &mut Grid, route: &RoutedEdge, style: Style) {
    let Some(anchor) = route.label_at else {
        return;
    };
    let width = UnicodeWidthStr::width(route.label.as_str()) as u16;
    let point = label_point(route, anchor, width);
    grid.write(point, &route.label, style);
}

fn label_point(route: &RoutedEdge, anchor: Point, width: u16) -> Point {
    if route
        .points
        .windows(2)
        .any(|pair| vertical_at(pair, anchor))
    {
        return Point {
            x: anchor.x.saturating_sub(width.saturating_add(1)),
            y: anchor.y,
        };
    }
    Point {
        x: anchor.x.saturating_sub(width / 2),
        y: anchor.y.saturating_sub(1),
    }
}

fn vertical_at(pair: &[Point], point: Point) -> bool {
    pair[0].x == pair[1].x
        && point.x == pair[0].x
        && point.y >= pair[0].y.min(pair[1].y)
        && point.y <= pair[0].y.max(pair[1].y)
}

fn arrow_markers(grid: &Grid, route: &RoutedEdge) -> Vec<ArrowMarker> {
    let mut markers = Vec::new();
    for pair in route.points.windows(2) {
        periodic_markers(grid, pair[0], pair[1], &mut markers);
    }
    if let Some(pair) = route.points.windows(2).last() {
        push_unique(&mut markers, marker_before(pair[0], pair[1]));
    }
    markers
}

fn periodic_markers(grid: &Grid, from: Point, to: Point, markers: &mut Vec<ArrowMarker>) {
    let distance = from.x.abs_diff(to.x) + from.y.abs_diff(to.y);
    let mut offset = ARROW_INTERVAL;
    while offset.saturating_add(END_CLEARANCE) < distance {
        let marker = marker_at(from, to, offset);
        if marker_cell_is_clear(grid, marker.point) {
            push_unique(markers, marker);
        }
        offset = offset.saturating_add(ARROW_INTERVAL);
    }
}

fn marker_cell_is_clear(grid: &Grid, point: Point) -> bool {
    grid.cell(point.x, point.y)
        .is_some_and(|cell| cell.line_layer != LineLayer::Structure && cell.glyph == ' ')
}

fn marker_before(from: Point, to: Point) -> ArrowMarker {
    let distance = from.x.abs_diff(to.x) + from.y.abs_diff(to.y);
    marker_at(from, to, distance.saturating_sub(1))
}

fn marker_at(from: Point, to: Point, offset: u16) -> ArrowMarker {
    ArrowMarker {
        point: advance(from, to, offset),
        direction: direction(from, to),
    }
}

fn advance(from: Point, to: Point, offset: u16) -> Point {
    let x = if to.x >= from.x {
        from.x.saturating_add(offset.min(from.x.abs_diff(to.x)))
    } else {
        from.x.saturating_sub(offset.min(from.x.abs_diff(to.x)))
    };
    let used = from.x.abs_diff(x);
    let y_offset = offset.saturating_sub(used);
    let y = if to.y >= from.y {
        from.y.saturating_add(y_offset)
    } else {
        from.y.saturating_sub(y_offset)
    };
    Point { x, y }
}

fn direction(from: Point, to: Point) -> ArrowDirection {
    if to.x > from.x {
        ArrowDirection::East
    } else if to.x < from.x {
        ArrowDirection::West
    } else if to.y > from.y {
        ArrowDirection::South
    } else {
        ArrowDirection::North
    }
}

fn push_unique(markers: &mut Vec<ArrowMarker>, marker: ArrowMarker) {
    if !markers.iter().any(|item| item.point == marker.point) {
        markers.push(marker);
    }
}

#[cfg(test)]
fn pt(x: u16, y: u16) -> Point {
    Point { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EdgeKind;

    #[test]
    fn vertical_edge_labels_are_placed_left_of_the_route() {
        let route = RoutedEdge {
            id: "no".into(),
            points: vec![pt(5, 1), pt(5, 5)],
            label: "NO".into(),
            label_at: Some(pt(5, 3)),
            kind: EdgeKind::Failure,
        };
        assert_eq!(label_point(&route, pt(5, 3), 2), pt(2, 3));
    }

    #[test]
    fn long_segments_receive_periodic_and_terminal_arrows() {
        let route = RoutedEdge {
            id: "long".into(),
            points: vec![pt(1, 1), pt(25, 1)],
            label: String::new(),
            label_at: None,
            kind: EdgeKind::Forward,
        };
        let markers = arrow_markers(&Grid::new(30, 3), &route);
        let points = markers
            .iter()
            .map(|marker| marker.point)
            .collect::<Vec<_>>();
        assert_eq!(points, vec![pt(9, 1), pt(17, 1), pt(24, 1)]);
        assert!(markers
            .iter()
            .all(|marker| marker.direction == ArrowDirection::East));
    }
}
