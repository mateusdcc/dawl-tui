use std::collections::HashMap;
use std::fmt::Write;

use crate::canvas::{Grid, EAST, NORTH, SOUTH, WEST};
use crate::theme::{Palette, Rgb, Style};

pub(super) fn render(grid: &Grid, cell_width: u16, cell_height: u16) -> String {
    let palette = Palette::midnight();
    let width = u32::from(grid.width) * u32::from(cell_width);
    let height = u32::from(grid.height) * u32::from(cell_height);
    let mut output = header(width, height, palette.background());
    write_lines(&mut output, grid, cell_width, cell_height, palette);
    write_diagonals(&mut output, grid, cell_width, cell_height, palette);
    super::svg_arrows::write(&mut output, grid, cell_width, cell_height, palette);
    for y in 0..grid.height {
        write_row(&mut output, grid, y, cell_width, cell_height, palette);
    }
    output.push_str("</svg>\n");
    output
}

fn write_diagonals(out: &mut String, grid: &Grid, cw: u16, ch: u16, palette: Palette) {
    let mut paths = HashMap::<Style, String>::new();
    for line in &grid.diagonals {
        let path = paths.entry(line.style).or_default();
        let from = cell_center(line.from, cw, ch);
        let to = cell_center(line.to, cw, ch);
        let _ = write!(path, "M{} {}L{} {}", from.0, from.1, to.0, to.1);
    }
    for (style, path) in paths {
        let color = hex(palette.foreground(style));
        let _ = writeln!(out, "<path d=\"{path}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1\" stroke-linecap=\"square\" stroke-linejoin=\"miter\"/>");
    }
}

fn cell_center(point: crate::model::Point, cw: u16, ch: u16) -> (u32, u32) {
    (
        u32::from(point.x) * u32::from(cw) + u32::from(cw) / 2,
        u32::from(point.y) * u32::from(ch) + u32::from(ch) / 2,
    )
}

fn write_lines(out: &mut String, grid: &Grid, cw: u16, ch: u16, palette: Palette) {
    let mut paths = HashMap::<Style, String>::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            append_cell_lines(&mut paths, grid, x, y, cw, ch);
        }
    }
    for (style, path) in paths {
        let color = hex(palette.foreground(style));
        let _ = writeln!(out, "<path d=\"{path}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1\" stroke-linecap=\"butt\" stroke-linejoin=\"miter\"/>");
    }
}

fn append_cell_lines(
    paths: &mut HashMap<Style, String>,
    grid: &Grid,
    x: u16,
    y: u16,
    cw: u16,
    ch: u16,
) {
    let Some(cell) = grid.cell(x, y).filter(|cell| cell.line != 0) else {
        return;
    };
    let left = u32::from(x) * u32::from(cw);
    let top = u32::from(y) * u32::from(ch);
    let center = (left + u32::from(cw) / 2, top + u32::from(ch) / 2);
    let path = paths.entry(cell.style).or_default();
    append_segment(path, cell.line & NORTH != 0, center, (center.0, top));
    append_segment(
        path,
        cell.line & EAST != 0,
        center,
        (left + u32::from(cw), center.1),
    );
    append_segment(
        path,
        cell.line & SOUTH != 0,
        center,
        (center.0, top + u32::from(ch)),
    );
    append_segment(path, cell.line & WEST != 0, center, (left, center.1));
}

fn append_segment(path: &mut String, present: bool, from: (u32, u32), to: (u32, u32)) {
    if present {
        let _ = write!(path, "M{} {}L{} {}", from.0, from.1, to.0, to.1);
    }
}

fn write_row(out: &mut String, grid: &Grid, y: u16, cw: u16, ch: u16, palette: Palette) {
    let mut x = 0;
    while x < grid.width {
        let Some(cell) = grid.cell(x, y) else { break };
        let (style, start) = (cell.style, x);
        let mut content = String::new();
        while x < grid.width && grid.cell(x, y).is_some_and(|item| item.style == style) {
            append_cell(&mut content, grid, x, y);
            x = x.saturating_add(1);
        }
        let width = u32::from(grid.width) * u32::from(cw);
        write_run(out, (start, y), (cw, ch), width, style, palette, &content);
    }
}

fn append_cell(content: &mut String, grid: &Grid, x: u16, y: u16) {
    if let Some(cell) = grid.cell(x, y).filter(|cell| !cell.continuation) {
        content.push(cell.glyph);
    }
}

#[allow(clippy::too_many_arguments)]
fn write_run(
    out: &mut String,
    cell: (u16, u16),
    size: (u16, u16),
    width: u32,
    style: Style,
    palette: Palette,
    content: &str,
) {
    if content.trim().is_empty() {
        return;
    }
    let title = style == Style::Title && cell.1 == 0;
    let px = if title {
        width / 2
    } else {
        u32::from(cell.0) * u32::from(size.0)
    };
    let py = if title {
        u32::from(size.1) + 6
    } else {
        (u32::from(cell.1) + 1) * u32::from(size.1) - u32::from(size.1 / 5)
    };
    let base_font_size = size.0.saturating_mul(5) / 3;
    let font_size = if title {
        base_font_size.saturating_add(6)
    } else if style == Style::Title {
        base_font_size.saturating_add(2)
    } else {
        base_font_size
    };
    let anchor = if title { " text-anchor=\"middle\"" } else { "" };
    let weight = if style == Style::Title { 700 } else { 500 };
    let color = hex(palette.foreground(style));
    let escaped = escape(content);
    let _ = writeln!(out, "<text x=\"{px}\" y=\"{py}\" fill=\"{color}\" font-family=\"ui-monospace, SFMono-Regular, Menlo, Consolas, monospace\" font-size=\"{font_size}\" font-weight=\"{weight}\" font-kerning=\"none\" font-variant-ligatures=\"none\"{anchor} xml:space=\"preserve\">{escaped}</text>");
}

fn header(width: u32, height: u32, background: Rgb) -> String {
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">\n<rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n", hex(background))
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn hex(color: Rgb) -> String {
    format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
}
