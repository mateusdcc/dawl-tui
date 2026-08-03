use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::model::{Point, Size};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    point: Point,
    axis: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Candidate {
    estimate: u32,
    cost: u32,
    state: State,
}

pub(super) struct Pathfinder<'a> {
    size: Size,
    blocked: &'a HashSet<Point>,
    used: &'a HashSet<Point>,
}

impl<'a> Pathfinder<'a> {
    pub(super) fn new(size: Size, blocked: &'a HashSet<Point>, used: &'a HashSet<Point>) -> Self {
        Self {
            size,
            blocked,
            used,
        }
    }

    pub(super) fn find(&self, start: Point, end: Point) -> Option<Vec<Point>> {
        let initial = State {
            point: start,
            axis: 0,
        };
        let mut queue = BinaryHeap::from([Candidate::new(initial, 0, end)]);
        let mut costs = HashMap::from([(initial, 0)]);
        let mut parents = HashMap::new();
        while let Some(candidate) = queue.pop() {
            if candidate.state.point == end {
                return rebuild(candidate.state, initial, &parents);
            }
            self.visit(candidate, end, &mut queue, &mut costs, &mut parents);
        }
        None
    }

    fn visit(
        &self,
        current: Candidate,
        end: Point,
        queue: &mut BinaryHeap<Candidate>,
        costs: &mut HashMap<State, u32>,
        parents: &mut HashMap<State, State>,
    ) {
        if costs
            .get(&current.state)
            .is_some_and(|cost| current.cost > *cost)
        {
            return;
        }
        for next in self.neighbors(current.state, end) {
            let cost = current
                .cost
                .saturating_add(self.step_cost(current.state, next));
            if costs.get(&next).is_some_and(|known| *known <= cost) {
                continue;
            }
            costs.insert(next, cost);
            parents.insert(next, current.state);
            queue.push(Candidate::new(next, cost, end));
        }
    }

    fn neighbors(&self, state: State, end: Point) -> Vec<State> {
        moves(state.point)
            .into_iter()
            .filter_map(|(point, axis)| {
                if !self.in_bounds(point) || (point != end && self.blocked.contains(&point)) {
                    return None;
                }
                Some(State { point, axis })
            })
            .collect()
    }

    fn step_cost(&self, from: State, to: State) -> u32 {
        let bend = if from.axis != 0 && from.axis != to.axis {
            6
        } else {
            0
        };
        let overlap = if self.used.contains(&to.point) { 12 } else { 0 };
        let proximity = adjacent(to.point)
            .iter()
            .filter(|point| self.blocked.contains(point))
            .count() as u32;
        1 + bend + overlap + proximity
    }

    fn in_bounds(&self, point: Point) -> bool {
        point.x < self.size.width && point.y < self.size.height
    }
}

impl Candidate {
    fn new(state: State, cost: u32, end: Point) -> Self {
        Self {
            estimate: cost.saturating_add(manhattan(state.point, end)),
            cost,
            state,
        }
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimate
            .cmp(&self.estimate)
            .then_with(|| other.cost.cmp(&self.cost))
            .then_with(|| other.state.point.y.cmp(&self.state.point.y))
            .then_with(|| other.state.point.x.cmp(&self.state.point.x))
            .then_with(|| other.state.axis.cmp(&self.state.axis))
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn rebuild(
    mut current: State,
    start: State,
    parents: &HashMap<State, State>,
) -> Option<Vec<Point>> {
    let mut points = vec![current.point];
    while current != start {
        current = *parents.get(&current)?;
        points.push(current.point);
    }
    points.reverse();
    Some(points)
}

fn moves(p: Point) -> [(Point, u8); 4] {
    [
        (pt(p.x.saturating_add(1), p.y), 1),
        (pt(p.x.saturating_sub(1), p.y), 1),
        (pt(p.x, p.y.saturating_add(1)), 2),
        (pt(p.x, p.y.saturating_sub(1)), 2),
    ]
}

fn pt(x: u16, y: u16) -> Point {
    Point { x, y }
}

fn adjacent(point: Point) -> [Point; 4] {
    moves(point).map(|item| item.0)
}

fn manhattan(a: Point, b: Point) -> u32 {
    u32::from(a.x.abs_diff(b.x)) + u32::from(a.y.abs_diff(b.y))
}
