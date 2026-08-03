use std::collections::HashSet;

use crate::model::Point;

const HORIZONTAL: u8 = 1;
const VERTICAL: u8 = 2;

#[derive(Default)]
pub(super) struct TrackGrid {
    horizontal: HashSet<Point>,
    vertical: HashSet<Point>,
    horizontal_cells: HashSet<Point>,
    vertical_cells: HashSet<Point>,
}

impl TrackGrid {
    pub(super) fn reserve(&mut self, points: &[Point]) {
        for pair in points.windows(2) {
            let mut current = pair[0];
            while current != pair[1] {
                let next = step_toward(current, pair[1]);
                self.reserve_atom(current, next);
                current = next;
            }
        }
    }

    pub(super) fn overlap_cost(&self, from: Point, to: Point) -> u32 {
        let Some((axis, origin)) = atom(from, to) else {
            return 0;
        };
        let occupied = match axis {
            HORIZONTAL => self.horizontal.contains(&origin),
            VERTICAL => self.vertical.contains(&origin),
            _ => false,
        };
        u32::from(occupied) * 1_000
    }

    pub(super) fn parallel_clearance_cost(&self, from: Point, to: Point) -> u32 {
        let Some((axis, origin)) = atom(from, to) else {
            return 0;
        };
        shifted_origins(axis, origin)
            .into_iter()
            .flatten()
            .filter(|point| match axis {
                HORIZONTAL => self.horizontal.contains(point),
                VERTICAL => self.vertical.contains(point),
                _ => false,
            })
            .count() as u32
            * 40
    }

    pub(super) fn crossing_cost(&self, point: Point, axis: u8) -> u32 {
        let crosses = match axis {
            HORIZONTAL => self.vertical_cells.contains(&point),
            VERTICAL => self.horizontal_cells.contains(&point),
            _ => false,
        };
        u32::from(crosses) * 8
    }

    fn reserve_atom(&mut self, from: Point, to: Point) {
        let Some((axis, origin)) = atom(from, to) else {
            return;
        };
        match axis {
            HORIZONTAL => {
                self.horizontal.insert(origin);
                self.horizontal_cells.extend([from, to]);
            }
            VERTICAL => {
                self.vertical.insert(origin);
                self.vertical_cells.extend([from, to]);
            }
            _ => {}
        }
    }
}

fn atom(from: Point, to: Point) -> Option<(u8, Point)> {
    horizontal_atom(from, to).or_else(|| vertical_atom(from, to))
}

fn horizontal_atom(from: Point, to: Point) -> Option<(u8, Point)> {
    (from.y == to.y && from.x.abs_diff(to.x) == 1).then_some((
        HORIZONTAL,
        Point {
            x: from.x.min(to.x),
            y: from.y,
        },
    ))
}

fn vertical_atom(from: Point, to: Point) -> Option<(u8, Point)> {
    (from.x == to.x && from.y.abs_diff(to.y) == 1).then_some((
        VERTICAL,
        Point {
            x: from.x,
            y: from.y.min(to.y),
        },
    ))
}

fn shifted_origins(axis: u8, point: Point) -> [Option<Point>; 2] {
    match axis {
        HORIZONTAL => [
            point.y.checked_sub(1).map(|y| Point { x: point.x, y }),
            point.y.checked_add(1).map(|y| Point { x: point.x, y }),
        ],
        VERTICAL => [
            point.x.checked_sub(1).map(|x| Point { x, y: point.y }),
            point.x.checked_add(1).map(|x| Point { x, y: point.y }),
        ],
        _ => [None, None],
    }
}

fn step_toward(from: Point, to: Point) -> Point {
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

fn advance(value: u16, target: u16) -> u16 {
    if value < target {
        value.saturating_add(1)
    } else {
        value.saturating_sub(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinguishes_collinear_overlap_from_perpendicular_crossing() {
        let mut tracks = TrackGrid::default();
        tracks.reserve(&[Point { x: 1, y: 2 }, Point { x: 4, y: 2 }]);
        assert_eq!(
            tracks.overlap_cost(Point { x: 2, y: 2 }, Point { x: 3, y: 2 }),
            1_000
        );
        assert_eq!(
            tracks.overlap_cost(Point { x: 3, y: 1 }, Point { x: 3, y: 2 }),
            0
        );
        assert_eq!(tracks.crossing_cost(Point { x: 3, y: 2 }, VERTICAL), 8);
    }

    #[test]
    fn penalizes_parallel_tracks_one_cell_apart() {
        let mut tracks = TrackGrid::default();
        tracks.reserve(&[Point { x: 1, y: 2 }, Point { x: 4, y: 2 }]);
        assert_eq!(
            tracks.parallel_clearance_cost(Point { x: 2, y: 3 }, Point { x: 3, y: 3 }),
            40
        );
    }
}
