use unicode_width::UnicodeWidthChar;

use crate::model::Point;
use crate::theme::Style;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
    pub glyph: char,
    pub style: Style,
    pub line: u8,
    pub continuation: bool,
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}

const NORTH: u8 = 1;
const EAST: u8 = 2;
const SOUTH: u8 = 4;
const WEST: u8 = 8;

impl Default for Cell {
    fn default() -> Self {
        Self { glyph: ' ', style: Style::Plain, line: 0, continuation: false }
    }
}

impl Rect {
    pub fn right(self) -> u16 { self.x.saturating_add(self.width.saturating_sub(1)) }
    pub fn bottom(self) -> u16 { self.y.saturating_add(self.height.saturating_sub(1)) }
    pub fn contains(self, point: Point) -> bool { point.x >= self.x && point.x <= self.right() && point.y >= self.y && point.y <= self.bottom() }
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        let len = usize::from(width) * usize::from(height);
        Self { width, height, cells: vec![Cell::default(); len] }
    }

    pub fn cell(&self, x: u16, y: u16) -> Option<&Cell> {
        self.index(x, y).and_then(|index| self.cells.get(index))
    }

    pub fn visible_char(&self, x: u16, y: u16) -> char {
        self.cell(x, y).map_or(' ', resolve)
    }

    pub fn write(&mut self, point: Point, text: &str, style: Style) {
        let mut x = point.x;
        for glyph in text.chars() {
            x = self.write_glyph(x, point.y, glyph, style);
        }
    }

    pub fn put(&mut self, point: Point, glyph: char, style: Style) {
        if let Some(cell) = self.cell_mut(point.x, point.y) {
            *cell = Cell { glyph, style, line: 0, continuation: false };
        }
    }

    pub fn draw_box(&mut self, rect: Rect, style: Style, dashed: bool) {
        if rect.width < 2 || rect.height < 2 { return; }
        self.draw_box_edges(rect, style, dashed);
        self.add_line(rect.x, rect.y, EAST | SOUTH, style);
        self.add_line(rect.right(), rect.y, WEST | SOUTH, style);
        self.add_line(rect.x, rect.bottom(), EAST | NORTH, style);
        self.add_line(rect.right(), rect.bottom(), WEST | NORTH, style);
    }


    fn draw_box_edges(&mut self, rect: Rect, style: Style, dashed: bool) {
        let left = rect.x.saturating_add(1);
        let right = rect.right().saturating_sub(1);
        let top = rect.y.saturating_add(1);
        let bottom = rect.bottom().saturating_sub(1);
        if left <= right { self.draw_horizontal(left, right, rect.y, style, dashed); }
        if left <= right { self.draw_horizontal(left, right, rect.bottom(), style, dashed); }
        if top <= bottom { self.draw_vertical(top, bottom, rect.x, style, dashed); }
        if top <= bottom { self.draw_vertical(top, bottom, rect.right(), style, dashed); }
    }

    pub fn draw_path(&mut self, points: &[Point], style: Style) {
        for pair in points.windows(2) {
            self.segment(pair[0], pair[1], style);
        }
    }

    pub fn arrow(&mut self, point: Point, style: Style) {
        self.put(point, '▶', style);
    }

    fn write_glyph(&mut self, x: u16, y: u16, glyph: char, style: Style) -> u16 {
        let width = UnicodeWidthChar::width(glyph).unwrap_or(0) as u16;
        if width == 0 { return x; }
        self.put(Point { x, y }, glyph, style);
        if width == 2 { self.mark_continuation(x.saturating_add(1), y, style); }
        x.saturating_add(width)
    }

    fn mark_continuation(&mut self, x: u16, y: u16, style: Style) {
        if let Some(cell) = self.cell_mut(x, y) {
            *cell = Cell { glyph: ' ', style, line: 0, continuation: true };
        }
    }

    fn draw_horizontal(&mut self, start: u16, end: u16, y: u16, style: Style, dashed: bool) {
        for x in start..=end {
            if !dashed || x % 2 == start % 2 { self.add_line(x, y, EAST | WEST, style); }
        }
    }

    fn draw_vertical(&mut self, start: u16, end: u16, x: u16, style: Style, dashed: bool) {
        for y in start..=end {
            if !dashed || y % 2 == start % 2 { self.add_line(x, y, NORTH | SOUTH, style); }
        }
    }

    fn segment(&mut self, from: Point, to: Point, style: Style) {
        if from.x == to.x { self.vertical_segment(from, to, style); }
        if from.y == to.y { self.horizontal_segment(from, to, style); }
    }

    fn horizontal_segment(&mut self, from: Point, to: Point, style: Style) {
        let (start, end) = ordered(from.x, to.x);
        for x in start..=end { self.add_line(x, from.y, EAST | WEST, style); }
    }

    fn vertical_segment(&mut self, from: Point, to: Point, style: Style) {
        let (start, end) = ordered(from.y, to.y);
        for y in start..=end { self.add_line(from.x, y, NORTH | SOUTH, style); }
    }

    fn add_line(&mut self, x: u16, y: u16, mask: u8, style: Style) {
        if let Some(cell) = self.cell_mut(x, y) {
            cell.line |= mask;
            cell.style = style;
        }
    }

    fn cell_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.index(x, y).and_then(|index| self.cells.get_mut(index))
    }

    fn index(&self, x: u16, y: u16) -> Option<usize> {
        if x >= self.width || y >= self.height { return None; }
        Some(usize::from(y) * usize::from(self.width) + usize::from(x))
    }
}

fn ordered(a: u16, b: u16) -> (u16, u16) { if a <= b { (a, b) } else { (b, a) } }

fn resolve(cell: &Cell) -> char {
    if cell.glyph != ' ' || cell.continuation { return cell.glyph; }
    match cell.line & 15 {
        1 | 4 | 5 => '│', 2 | 8 | 10 => '─', 3 => '└', 6 => '┌',
        9 => '┘', 12 => '┐', 7 => '├', 11 => '┴', 13 => '┤', 14 => '┬',
        15 => '┼', _ => ' ',
    }
}
