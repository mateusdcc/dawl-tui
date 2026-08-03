use std::collections::HashMap;
use std::fmt::Write;

use crate::canvas::{ArrowDirection, Grid};
use crate::theme::{Palette, Style};

pub(super) fn write(out: &mut String, grid: &Grid, cw: u16, ch: u16, palette: Palette) {
    for (style, path) in arrow_paths(grid, cw, ch) {
        let color = hex(palette.foreground(style));
        let _ = writeln!(out, "<path d=\"{path}\" fill=\"{color}\" stroke=\"none\"/>");
    }
}

fn arrow_paths(grid: &Grid, cw: u16, ch: u16) -> HashMap<Style, String> {
    let mut paths = HashMap::<Style, String>::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            append_cell_arrow(&mut paths, grid, x, y, cw, ch);
        }
    }
    paths
}

fn append_cell_arrow(
    paths: &mut HashMap<Style, String>,
    grid: &Grid,
    x: u16,
    y: u16,
    cw: u16,
    ch: u16,
) {
    let Some(cell) = grid
        .cell(x, y)
        .filter(|cell| cell.arrow != ArrowDirection::None)
    else {
        return;
    };
    append_triangle(
        paths.entry(cell.style).or_default(),
        x,
        y,
        cw,
        ch,
        cell.arrow,
    );
}

fn append_triangle(path: &mut String, x: u16, y: u16, cw: u16, ch: u16, arrow: ArrowDirection) {
    let cx = u32::from(x) * u32::from(cw) + u32::from(cw) / 2;
    let cy = u32::from(y) * u32::from(ch) + u32::from(ch) / 2;
    let dx = u32::from(cw / 2);
    let dy = u32::from(ch / 3);
    let points = triangle_points((cx, cy), dx, dy, arrow);
    let _ = write!(
        path,
        "M{} {}L{} {}L{} {}Z",
        points[0].0, points[0].1, points[1].0, points[1].1, points[2].0, points[2].1
    );
}

fn triangle_points(center: (u32, u32), dx: u32, dy: u32, arrow: ArrowDirection) -> [(u32, u32); 3] {
    let (cx, cy) = center;
    match arrow {
        ArrowDirection::North => [(cx, cy - dy), (cx - dx, cy + dy), (cx + dx, cy + dy)],
        ArrowDirection::East => [(cx + dx, cy), (cx - dx, cy - dy), (cx - dx, cy + dy)],
        ArrowDirection::South => [(cx, cy + dy), (cx - dx, cy - dy), (cx + dx, cy - dy)],
        ArrowDirection::West => [(cx - dx, cy), (cx + dx, cy - dy), (cx + dx, cy + dy)],
        ArrowDirection::None => [center; 3],
    }
}

fn hex(color: crate::theme::Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
}
