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

pub const NORTH: u8 = 1;
pub const EAST: u8 = 2;
pub const SOUTH: u8 = 4;
pub const WEST: u8 = 8;

impl Default for Cell {
    fn default() -> Self {
        Self {
            glyph: ' ',
            style: Style::Plain,
            line: 0,
            continuation: false,
        }
    }
}

impl Rect {
    pub fn right(self) -> u16 {
        self.x.saturating_add(self.width.saturating_sub(1))
    }
    pub fn bottom(self) -> u16 {
        self.y.saturating_add(self.height.saturating_sub(1))
    }
    pub fn contains(self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.right()
            && point.y >= self.y
            && point.y <= self.bottom()
    }
}

pub fn ordered(a: u16, b: u16) -> (u16, u16) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn resolve(cell: &Cell) -> char {
    if cell.glyph != ' ' || cell.continuation {
        return cell.glyph;
    }
    const GLYPHS: [char; 16] = [
        ' ', '│', '─', '└', '│', '│', '┌', '├', '─', '┘', '─', '┴', '┐', '┤', '┬', '┼',
    ];
    GLYPHS[usize::from(cell.line & 15)]
}
