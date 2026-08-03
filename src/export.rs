use std::fmt::Write;

use crate::canvas::Grid;
use crate::theme::{Palette, Style};

mod svg_arrows;
mod svg_format;

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
    svg_format::render(grid, cell_width, cell_height)
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
