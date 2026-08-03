use crate::model::Point;

use super::{EAST, NORTH, SOUTH, WEST};

pub(super) fn step_toward(from: Point, to: Point) -> Point {
    if from.x != to.x {
        return Point {
            x: advance(from.x, to.x),
            y: from.y,
        };
    }
    Point {
        x: from.x,
        y: advance(from.y, to.y),
    }
}

pub(super) fn direction_masks(from: Point, to: Point) -> (u8, u8) {
    match (from.x.cmp(&to.x), from.y.cmp(&to.y)) {
        (std::cmp::Ordering::Less, _) => (EAST, WEST),
        (std::cmp::Ordering::Greater, _) => (WEST, EAST),
        (_, std::cmp::Ordering::Less) => (SOUTH, NORTH),
        _ => (NORTH, SOUTH),
    }
}

fn advance(value: u16, target: u16) -> u16 {
    if value < target {
        value.saturating_add(1)
    } else {
        value.saturating_sub(1)
    }
}
