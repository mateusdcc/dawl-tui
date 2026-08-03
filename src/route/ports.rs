use crate::canvas::Rect;
use crate::model::{Edge, EdgeKind, Point, Port};

pub fn select_ports(edge: &Edge, from: Rect, to: Rect) -> (Port, Port) {
    if let (Some(a), Some(b)) = (edge.from_port, edge.to_port) {
        return (a, b);
    }
    if edge.kind == EdgeKind::Back {
        return (
            edge.from_port.unwrap_or(Port::South),
            edge.to_port.unwrap_or(Port::West),
        );
    }
    let defaults = directional_ports(from, to, edge.kind);
    (
        edge.from_port.unwrap_or(defaults.0),
        edge.to_port.unwrap_or(defaults.1),
    )
}

fn directional_ports(from: Rect, to: Rect, kind: EdgeKind) -> (Port, Port) {
    if kind == EdgeKind::Failure {
        return (Port::South, Port::West);
    }
    let dx = i32::from(center_x(to)) - i32::from(center_x(from));
    let dy = i32::from(center_y(to)) - i32::from(center_y(from));
    if dx.abs() >= dy.abs() {
        if dx >= 0 {
            (Port::East, Port::West)
        } else {
            (Port::West, Port::East)
        }
    } else if dy >= 0 {
        (Port::South, Port::North)
    } else {
        (Port::North, Port::South)
    }
}

pub fn port_point(rect: Rect, port: Port) -> Point {
    match port {
        Port::North => Point {
            x: center_x(rect),
            y: rect.y,
        },
        Port::East => Point {
            x: rect.right(),
            y: center_y(rect),
        },
        Port::South => Point {
            x: center_x(rect),
            y: rect.bottom(),
        },
        Port::West => Point {
            x: rect.x,
            y: center_y(rect),
        },
    }
}

pub fn center_x(rect: Rect) -> u16 {
    rect.x.saturating_add(rect.width / 2)
}

pub fn center_y(rect: Rect) -> u16 {
    rect.y.saturating_add(rect.height / 2)
}
