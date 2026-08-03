use dawl_tui::canvas::Grid;
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
    grid.write(Point { x: 0, y: 0 }, "<&", Style::Title);
    let output = export::svg(&grid, 9, 18);
    assert!(output.contains("width=\"36\""));
    assert!(output.contains("height=\"36\""));
    assert!(output.contains("&lt;&amp;"));
    assert!(!output.contains("<text><&"));
}
