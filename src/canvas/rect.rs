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
    pub line_layer: LineLayer,
    pub arrow: ArrowDirection,
    pub continuation: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ArrowDirection {
    #[default]
    None,
    North,
    East,
    South,
    West,
}

impl ArrowDirection {
    fn glyph(self) -> Option<char> {
        match self {
            Self::None => None,
            Self::North => Some('▲'),
            Self::East => Some('▶'),
            Self::South => Some('▼'),
            Self::West => Some('◀'),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LineLayer {
    #[default]
    None,
    Structure,
    Route,
}

pub const NORTH: u8 = 1;
pub const EAST: u8 = 2;
pub const SOUTH: u8 = 4;
pub const WEST: u8 = 8;
pub const DIAGONAL_RISING: u8 = 16;
pub const DIAGONAL_FALLING: u8 = 32;

impl Default for Cell {
    fn default() -> Self {
        Self {
            glyph: ' ',
            style: Style::Plain,
            line: 0,
            line_layer: LineLayer::None,
            arrow: ArrowDirection::None,
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

pub fn resolve(cell: &Cell) -> char {
    if cell.glyph != ' ' || cell.continuation {
        return cell.glyph;
    }
    if let Some(glyph) = cell.arrow.glyph() {
        return glyph;
    }
    resolve_line(cell.line)
}

fn resolve_line(line: u8) -> char {
    let orthogonal = line & 15;
    if orthogonal == 0 {
        return match line & (DIAGONAL_RISING | DIAGONAL_FALLING) {
            DIAGONAL_RISING => '╱',
            DIAGONAL_FALLING => '╲',
            value if value != 0 => '╳',
            _ => ' ',
        };
    }
    const GLYPHS: [char; 16] = [
        ' ', '│', '─', '└', '│', '│', '┌', '├', '─', '┘', '─', '┴', '┐', '┤', '┬', '┼',
    ];
    GLYPHS[usize::from(orthogonal)]
}
