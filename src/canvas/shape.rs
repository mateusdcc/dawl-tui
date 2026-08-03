use crate::canvas::{Rect, DIAGONAL_FALLING, DIAGONAL_RISING};
use crate::model::Point;
use crate::theme::Style;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Diagonal {
    pub from: Point,
    pub to: Point,
    pub style: Style,
}

pub(crate) fn diamond(rect: Rect, style: Style) -> [Diagonal; 4] {
    let [top, right, bottom, left] = vertices(rect);
    [
        edge(top, right, style),
        edge(right, bottom, style),
        edge(bottom, left, style),
        edge(left, top, style),
    ]
}

fn edge(from: Point, to: Point, style: Style) -> Diagonal {
    Diagonal { from, to, style }
}

fn vertices(rect: Rect) -> [Point; 4] {
    [top(rect), right(rect), bottom(rect), left(rect)]
}

fn top(rect: Rect) -> Point {
    Point {
        x: rect.x + rect.width / 2,
        y: rect.y,
    }
}

fn right(rect: Rect) -> Point {
    Point {
        x: rect.right(),
        y: rect.y + rect.height / 2,
    }
}

fn bottom(rect: Rect) -> Point {
    Point {
        x: top(rect).x,
        y: rect.bottom(),
    }
}

fn left(rect: Rect) -> Point {
    Point {
        x: rect.x,
        y: right(rect).y,
    }
}

pub(crate) fn raster_cells(line: Diagonal) -> Vec<(Point, u8)> {
    let dx = line.from.x.abs_diff(line.to.x);
    let dy = line.from.y.abs_diff(line.to.y);
    let steps = dx.max(dy).max(1);
    (0..=steps)
        .map(|step| (interpolate(line, step, steps), diagonal_mask(line)))
        .collect()
}

fn interpolate(line: Diagonal, step: u16, steps: u16) -> Point {
    Point {
        x: interpolate_axis(line.from.x, line.to.x, step, steps),
        y: interpolate_axis(line.from.y, line.to.y, step, steps),
    }
}

fn interpolate_axis(from: u16, to: u16, step: u16, steps: u16) -> u16 {
    let delta = from.abs_diff(to);
    let offset = (u32::from(delta) * u32::from(step) / u32::from(steps)) as u16;
    if to >= from {
        from + offset
    } else {
        from - offset
    }
}

fn diagonal_mask(line: Diagonal) -> u8 {
    if (line.to.x >= line.from.x) == (line.to.y >= line.from.y) {
        DIAGONAL_FALLING
    } else {
        DIAGONAL_RISING
    }
}
