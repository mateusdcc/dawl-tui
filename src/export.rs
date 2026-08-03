use std::fmt::Write;

use crate::canvas::Grid;
use crate::theme::{Palette, Rgb, Style};

pub fn text(grid: &Grid) -> String {
    (0..grid.height)
        .map(|y| text_line(grid, y))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn ansi(grid: &Grid) -> String {
    let palette = Palette::midnight();
    let mut output = String::new();
    for y in 0..grid.height {
        write_ansi_line(&mut output, grid, y, palette);
    }
    output
}

pub fn svg(grid: &Grid, cell_width: u16, cell_height: u16) -> String {
    let palette = Palette::midnight();
    let width = u32::from(grid.width) * u32::from(cell_width);
    let height = u32::from(grid.height) * u32::from(cell_height);
    let mut output = svg_header(width, height, palette.background());
    for y in 0..grid.height {
        write_svg_row(&mut output, grid, y, cell_width, cell_height, palette);
    }
    output.push_str("</svg>\n");
    output
}

fn text_line(grid: &Grid, y: u16) -> String {
    let mut line = String::new();
    for x in 0..grid.width {
        let Some(cell) = grid.cell(x, y) else {
            continue;
        };
        if !cell.continuation {
            line.push(grid.visible_char(x, y));
        }
    }
    line.trim_end_matches(' ').to_owned()
}

fn write_ansi_line(output: &mut String, grid: &Grid, y: u16, palette: Palette) {
    let mut current = None;
    for x in 0..grid.width {
        let Some(cell) = grid.cell(x, y) else {
            continue;
        };
        if cell.continuation {
            continue;
        }
        if current != Some(cell.style) {
            write_ansi_style(output, cell.style, palette);
        }
        current = Some(cell.style);
        output.push(grid.visible_char(x, y));
    }
    output.push_str("\u{1b}[0m\n");
}

fn write_ansi_style(output: &mut String, style: Style, palette: Palette) {
    let foreground = palette.foreground(style);
    let background = palette.background();
    let _ = write!(
        output,
        "\u{1b}[38;2;{};{};{}m\u{1b}[48;2;{};{};{}m",
        foreground.red,
        foreground.green,
        foreground.blue,
        background.red,
        background.green,
        background.blue
    );
}

fn svg_header(width: u32, height: u32, background: Rgb) -> String {
    format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">\n<rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n", hex(background))
}

fn write_svg_row(
    output: &mut String,
    grid: &Grid,
    y: u16,
    cell_width: u16,
    cell_height: u16,
    palette: Palette,
) {
    let mut x = 0;
    while x < grid.width {
        let Some(cell) = grid.cell(x, y) else {
            break;
        };
        let style = cell.style;
        let start = x;
        let mut content = String::new();
        while x < grid.width && grid.cell(x, y).is_some_and(|item| item.style == style) {
            append_svg_cell(&mut content, grid, x, y);
            x = x.saturating_add(1);
        }
        write_svg_run(
            output,
            start,
            y,
            cell_width,
            cell_height,
            style,
            palette,
            &content,
        );
    }
}

fn append_svg_cell(content: &mut String, grid: &Grid, x: u16, y: u16) {
    let Some(cell) = grid.cell(x, y) else {
        return;
    };
    if !cell.continuation {
        content.push(grid.visible_char(x, y));
    }
}

#[allow(clippy::too_many_arguments)]
fn write_svg_run(
    output: &mut String,
    x: u16,
    y: u16,
    cell_width: u16,
    cell_height: u16,
    style: Style,
    palette: Palette,
    content: &str,
) {
    if content.trim().is_empty() {
        return;
    }
    let px = u32::from(x) * u32::from(cell_width);
    let py = (u32::from(y) + 1) * u32::from(cell_height) - u32::from(cell_height / 5);
    let foreground = hex(palette.foreground(style));
    let escaped = escape(content);
    let _ = writeln!(output, "<text x=\"{px}\" y=\"{py}\" fill=\"{foreground}\" font-family=\"ui-monospace, SFMono-Regular, Menlo, Consolas, monospace\" font-size=\"{}\" xml:space=\"preserve\">{escaped}</text>", cell_height.saturating_sub(2));
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
