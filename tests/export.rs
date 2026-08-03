use dawl_tui::canvas::{ArrowDirection, Grid, Rect};
use dawl_tui::export;
use dawl_tui::model::Point;
use dawl_tui::theme::Style;

#[test]
fn text_export_trims_only_trailing_cells() {
    let mut grid = Grid::new(8, 2);
    grid.write(Point { x: 1, y: 0 }, "A B", Style::Plain);
    let output = export::text(&grid);
    assert_eq!(output, " A B\n");
    assert!(!output.contains("\u{1b}["));
}

#[test]
fn ansi_export_coalesces_adjacent_equal_styles() {
    let mut grid = Grid::new(4, 1);
    grid.write(Point { x: 0, y: 0 }, "ABC", Style::Agent);
    let output = export::ansi(&grid);
    assert!(output.contains("\u{1b}[38;2;91;168;255m"));
    assert_eq!(output.matches("38;2;91;168;255").count(), 1);
}

#[test]
fn svg_export_escapes_text_and_uses_cell_dimensions() {
    let mut grid = Grid::new(4, 2);
    grid.draw_path(&[Point { x: 0, y: 1 }, Point { x: 3, y: 1 }], Style::Edge);
    grid.write(Point { x: 0, y: 0 }, "<&", Style::Title);
    let output = export::svg(&grid, 9, 18);
    assert!(output.contains("width=\"36\""));
    assert!(output.contains("height=\"36\""));
    assert!(output.contains("&lt;&amp;"));
    assert!(output.contains("<path d=\""));
    assert!(!output.contains('─'));
    assert!(!output.contains("<text><&"));
}

#[test]
fn svg_export_draws_decisions_as_vector_diamonds() {
    let mut grid = Grid::new(13, 7);
    grid.draw_diamond(
        Rect {
            x: 2,
            y: 1,
            width: 9,
            height: 5,
        },
        Style::Decision,
    );
    grid.write(Point { x: 4, y: 3 }, "pass?", Style::Decision);
    let output = export::svg(&grid, 6, 10);
    assert!(output.contains("M39 15L63 35"));
    assert!(output.contains("font-size=\"10\""));
    assert!(!output.contains('◇'));
}

#[test]
fn svg_export_draws_arrowheads_as_centered_triangles() {
    let mut grid = Grid::new(5, 1);
    grid.draw_path(&[Point { x: 0, y: 0 }, Point { x: 4, y: 0 }], Style::Edge);
    grid.arrow(Point { x: 2, y: 0 }, ArrowDirection::East, Style::Edge);
    let output = export::svg(&grid, 6, 10);
    assert!(output.contains("M18 5L12 2L12 8Z"));
    assert!(!output.contains('▶'));
}
